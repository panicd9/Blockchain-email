mod blockchain_email;
use aes_gcm_siv::{
    aead::{Aead, KeyInit, OsRng},
    Aes256GcmSiv, Nonce,
};
use byteorder::{BigEndian, ByteOrder};
use chrono::{DateTime, Utc};
use colored::*;
use ecies::{decrypt, encrypt, PublicKey, SecretKey};
use ethers::{
    core::k256,
    middleware::SignerMiddleware,
    providers::{Http, Provider},
    signers::{LocalWallet, Signer, Wallet},
    types::{Address, Bytes, Chain, H160},
    utils,
};
use libsecp256k1::util::TAG_PUBKEY_FULL;
use blockchain_email::*;
use std::fmt;
use std::sync::Arc;
use term_size;

use crate::cli::NormalizeArgs;

const _ANVIL_RPC_URL: &str = "http://127.0.0.1:8545";
const _ANVIL_PRIVATE_KEY_0: &str =
    "ac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80";
const _ANVIL_PRIVATE_KEY_1: &str = "59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d";
const _ANVIL_ADDRESS_ACC_0: &str = "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266";
const _ANVIL_ADDRESS_ACC_1: &str = "0x70997970C51812dc3A010C7d01b50e0d17dc79C8";
const _ADDR_CONTRACT_STR: &str = "0x5FbDB2315678afecb367f032d93F642f64180aa3";
static _ADDR_CONTRACT: H160 = H160([
    0x5F, 0xBD, 0xB2, 0x31, 0x56, 0x78, 0xAF, 0xEC, 0xB3, 0x67, 0xF0, 0x32, 0xD9, 0x3F, 0x64, 0x2F,
    0x64, 0x18, 0x0A, 0xA3,
]);

// Prepend to public key to indicate uncompressed key
// const TAG_PUBKEY_FULL: u8 = 0x04;

type Client = SignerMiddleware<Provider<Http>, Wallet<k256::ecdsa::SigningKey>>;

#[derive(Debug)]
enum MessageType {
    Incoming,
    Outgoing,
}

#[derive(Debug)]
struct Message {
    sender: Address,
    content: String,
    timestamp: u64,
    message_type: MessageType,
}

// impl fmt::Display for Message {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         let arrow = match self.message_type {
//             MessageType::Incoming => "->".green(),
//             MessageType::Outgoing => "<-".red(),
//         };
//         let timestamp = DateTime::<Utc>::from_timestamp(self.timestamp as i64, 0)
//             .format("%Y-%m-%d %H:%M:%S")
//             .to_string();
//         write!(f, "{} {} {}: {}", timestamp, arrow, self.sender.to_string().blue(), self.content)
//     }
// }

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let arrow = match self.message_type {
            MessageType::Incoming => "------------------>".green(),
            MessageType::Outgoing => "<------------------".red(),
        };
        let timestamp = DateTime::<Utc>::from_timestamp(self.timestamp as i64, 0)
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S")
            .to_string();
        let sender = format!("Sender: {}", self.sender);
        let content = format!("Content: {}", self.content);

        let width = term_size::dimensions().unwrap_or((80, 24)).0;

        match self.message_type {
            MessageType::Incoming => {
                write!(
                    f,
                    "{:<width$}\n{:<width$}\n{:<width$}\n{:<width$}\n",
                    arrow,
                    timestamp,
                    sender,
                    content,
                    width = width
                )
            }
            MessageType::Outgoing => {
                write!(
                    f,
                    "{:>width$}\n{:>width$}\n{:>width$}\n{:>width$}\n",
                    arrow,
                    timestamp,
                    sender,
                    content,
                    width = width
                )
            }
        }
    }
}

// Merging messages optimized because input vecs are already sorted. It is O(n) instead of and O(n * log(n)) if we use Vec::sort()
fn merge_messages(mut vec1: Vec<Message>, mut vec2: Vec<Message>) -> Vec<Message> {
    let mut result = Vec::new();

    while !vec1.is_empty() && !vec2.is_empty() {
        if vec1[0].timestamp <= vec2[0].timestamp {
            result.push(vec1.remove(0));
        } else {
            result.push(vec2.remove(0));
        }
    }

    while !vec1.is_empty() {
        result.push(vec1.remove(0));
    }

    while !vec2.is_empty() {
        result.push(vec2.remove(0));
    }

    result
}

fn init_wallet(private_key: &str) -> LocalWallet {
    let wallet: LocalWallet = private_key
        .parse::<LocalWallet>()
        .unwrap()
        .with_chain_id(Chain::AnvilHardhat);

    // Second way to initialize wallet
    // let wallet_test: LocalWallet = LocalWallet::from_bytes(hex::decode(private_key).unwrap().as_slice()).unwrap().with_chain_id(Chain::AnvilHardhat);

    return wallet;
}

fn _init_contract(
    contract_address: Address,
    client: &Client,
) -> BlockchainEmail<SignerMiddleware<Provider<Http>, Wallet<k256::ecdsa::SigningKey>>> {
    let contract = blockchain_email::BlockchainEmail::new(contract_address, Arc::new(client.clone()));
    return contract;
}

async fn _send_message(
    contract_address: Address,
    private_key_string: &str,
    client: &Client,
    recipient: Address,
    message: &str,
    verbose: bool,
) {
    let contract = _init_contract(contract_address, client);

    // calculate public key from private key instead of getting it from contract
    // let pk = _get_public_key(contract_address, client, recipient, None).await;
    let sk = SecretKey::parse_slice(hex::decode(private_key_string).unwrap().as_slice()).unwrap();
    let pk = PublicKey::from_secret_key(&sk);
    let (sk, _) = (&sk.serialize(), &pk.serialize());

    let encrypted_chat_sk = contract
        .get_encrypted_secret_key(recipient)
        .call()
        .await
        .unwrap()
        .to_vec();

    if encrypted_chat_sk.len() == 0 {
        println!("No encrypted chat secret key found for chat with recipient {}. Initialize chat secret key first!", recipient);
        return;
    }

    if verbose {
        println!(
            "Encrypted chat secret key: {:?}, \nLength: {}",
            encrypted_chat_sk,
            encrypted_chat_sk.len()
        );
    }

    let chat_sk = decrypt(sk, &encrypted_chat_sk).unwrap();

    let cipher = Aes256GcmSiv::new_from_slice(&chat_sk).unwrap();

    let mut rng = OsRng;
    let mut nonce = [0u8; 12];
    ethers::core::rand::Rng::fill(&mut rng, &mut nonce[..]);

    // let nonce = Nonce::from_slice(b"unique nonce"); // 96-bits; unique per message
    let encrypted_message = cipher.encrypt(Nonce::from_slice(&nonce), message.as_bytes()).unwrap();

    let prepended_nonce_to_encrypted_message = [&nonce, encrypted_message.as_slice()].concat();

    if verbose {
        println!(
            "Encrypted message len: {:?}",
            prepended_nonce_to_encrypted_message.len()
        );
    }

    let tx_receipt = contract
        .send_message(
            recipient,
            Bytes::from_iter(prepended_nonce_to_encrypted_message),
        )
        .send()
        .await
        .unwrap()
        .await
        .unwrap()
        .unwrap();

    if verbose {
        println!(
            "Transaction receipt (send_message) : {}",
            serde_json::to_string_pretty(&tx_receipt).unwrap()
        );
    }
}

async fn _get_messages(
    contract_address: Address,
    client: Client,
    private_key_string: &str,
    recipient: Address,
    verbose: bool,
) -> Vec<Message> {
    let contract = _init_contract(contract_address, &client);

    let sk = SecretKey::parse_slice(hex::decode(private_key_string).unwrap().as_slice()).unwrap();
    let pk: PublicKey = PublicKey::from_secret_key(&sk);
    let (sk, pk) = (&sk.serialize(), &pk.serialize());

    let sender: Address = Address::from_slice(&utils::keccak256(&pk[1..65])[12..]);

    let encrypted_chat_sk = contract
        .get_encrypted_secret_key(recipient)
        .call()
        .await
        .unwrap()
        .to_vec();

    let decrypted_chat_sk = decrypt(sk, &encrypted_chat_sk).unwrap();

    let encrypted_messages_from_sender = contract
        .get_messages(sender, recipient)
        .call()
        .await
        .unwrap();

    if verbose {
        println!(
            "Encrypted chat secret key: {:?}, \nLength: {}",
            encrypted_chat_sk,
            encrypted_chat_sk.len()
        );
    }

    // Messages I sent to recipient
    let mut messages_from_sender: Vec<Message> = vec![];

    for encrypted_message_with_nonce_and_timestamp in &encrypted_messages_from_sender {
        let nonce = Nonce::from_slice(&encrypted_message_with_nonce_and_timestamp[0..12]);
        let len = encrypted_message_with_nonce_and_timestamp.len();
        let encrypted_message = &encrypted_message_with_nonce_and_timestamp[12..len - 32];
        let timestamp_bytes = &encrypted_message_with_nonce_and_timestamp[len - 32..];
        let timestamp = BigEndian::read_u64(&timestamp_bytes[24..32]);

        let cipher = Aes256GcmSiv::new_from_slice(&decrypted_chat_sk).unwrap();
        let message_text = cipher.decrypt(nonce, encrypted_message).unwrap();

        let message = Message {
            message_type: MessageType::Outgoing,
            sender: sender,
            content: String::from_utf8(message_text).unwrap(),
            timestamp: timestamp,
        };

        messages_from_sender.push(message);
    }

    // Messages recipient sent to me
    let encrypted_messages_from_recipient = contract
        .get_messages(recipient, sender)
        .call()
        .await
        .unwrap();

    let mut messages_from_recipient: Vec<Message> = vec![];

    for encrypted_message_with_nonce_and_timestamp in &encrypted_messages_from_recipient {
        let nonce = Nonce::from_slice(&encrypted_message_with_nonce_and_timestamp[0..12]);
        let len = encrypted_message_with_nonce_and_timestamp.len();
        let encrypted_message = &encrypted_message_with_nonce_and_timestamp[12..len - 32];
        let timestamp_bytes = &encrypted_message_with_nonce_and_timestamp[len - 32..];
        let timestamp = BigEndian::read_u64(&timestamp_bytes[24..32]);
        let cipher = Aes256GcmSiv::new_from_slice(&decrypted_chat_sk).unwrap();
        let message_text = cipher.decrypt(nonce, encrypted_message).unwrap();

        let message = Message {
            message_type: MessageType::Incoming,
            sender: recipient,
            content: String::from_utf8(message_text).unwrap(),
            timestamp: timestamp,
        };

        messages_from_recipient.push(message);
    }

    let messages = merge_messages(messages_from_sender, messages_from_recipient);

    return messages;
}

async fn _add_public_key(
    contract_address: Address,
    client: Client,
    private_key_string: &str,
    verbose: bool,
) {
    let contract = _init_contract(contract_address, &client);

    let sk = SecretKey::parse_slice(hex::decode(private_key_string).unwrap().as_slice()).unwrap();
    let pk: PublicKey = PublicKey::from_secret_key(&sk);
    let (_, pk) = (&sk.serialize(), &pk.serialize());

    // Remmove first byte (TAG_PUBKEY_FULL = 4) from public key
    let public_key_half_1: [u8; 32] = pk[1..33].try_into().expect("Slice with incorrect length");
    let public_key_half_2: [u8; 32] = pk[34..65].try_into().expect("Slice with incorrect length");

    let tx_receipt = contract
        .add_public_key(public_key_half_1, public_key_half_2)
        .send()
        .await
        .unwrap()
        .await
        .unwrap()
        .unwrap();

    if verbose {
        println!(
            "Transaction receipt (add_public_key) : {}",
            serde_json::to_string_pretty(&tx_receipt).unwrap()
        );
    }
}

async fn _get_public_key(
    contract_address: Address,
    client: &Client,
    address: Address,
    messanger_contract: Option<
    BlockchainEmail<SignerMiddleware<Provider<Http>, Wallet<k256::ecdsa::SigningKey>>>,
    >,
) -> Option<[u8; 65]> {
    let contract = messanger_contract.unwrap_or_else(|| _init_contract(contract_address, &client));

    let (public_key_half_1, public_key_half_2): ([u8; 32], [u8; 32]) =
        contract.get_public_key(address).call().await.unwrap();

    let mut public_key: [u8; 65] = [0; 65];
    // This is not needed because parse_slice() in encrypt() function reconstructs full key but I kept it to keep consistency
    public_key[0] = TAG_PUBKEY_FULL;
    public_key[1..33].copy_from_slice(&public_key_half_1);
    public_key[33..].copy_from_slice(&public_key_half_2);

    if public_key[1..].iter().all(|&byte| byte == 0) {
        return None;
    } else {
        return Some(public_key);
    }
}

async fn _add_encrypted_secret_keys(
    contract_address: Address,
    private_key_string: &str,
    client: &Client,
    receiver: Address,
    verbose: bool,
) {
    let contract = _init_contract(contract_address, &client);

    let receiver_pk =
        &_get_public_key(contract_address, client, receiver, Some(contract.clone())).await;

    match receiver_pk {
        None => {
            println!("Public key for Receiver is not set!");
            return;
        }
        Some(receiver_pk) => {
            if verbose {
                println!("Receiver public key is: {:?}", receiver_pk);
            }
        }
    }
    let receiver_pk = &receiver_pk.unwrap();

    let sender_sk =
        SecretKey::parse_slice(hex::decode(private_key_string).unwrap().as_slice()).unwrap();
    let sender_pk: PublicKey = PublicKey::from_secret_key(&sender_sk);
    let (_, sender_pk) = (&sender_sk.serialize(), &sender_pk.serialize());

    let key: aes_gcm_siv::aead::generic_array::GenericArray<u8,_>  = Aes256GcmSiv::generate_key(&mut OsRng);
    // let cipher: Aes256GcmSiv = Aes256GcmSiv::new(&key);
    // let key_bytes = &[
    //     13, 140, 250, 52, 48, 152, 33, 244, 36, 77, 85, 83, 14, 4, 251, 85, 29, 196, 64, 43, 37,
    //     243, 30, 217, 206, 6, 43, 226, 21, 214, 247, 197,
    // ];

    let encrypted_sender_sk_vec = encrypt(sender_pk, &key).unwrap();
    // let encrypted_sender_sk_slice = encrypted_sender_sk_vec.as_slice();

    let encrypted_receiver_sk_vec = encrypt(receiver_pk, &key).unwrap();
    // let encrypted_receiver_sk_slice = encrypted_receiver_sk_vec.as_slice();

    let tx_receipt = contract
        .add_encrypted_secret_keys(
            receiver,
            Bytes::from_iter(encrypted_sender_sk_vec),
            Bytes::from_iter(encrypted_receiver_sk_vec),
        )
        .send()
        .await
        .unwrap_or_else(|_| {
            eprintln!("Chat secret key is already added! Send or get messages instead.");
            std::process::exit(1);
        })
        .await
        .unwrap()
        .unwrap();

    if verbose {
        println!(
            "Transaction receipt (add_encrypted_secret_keys) : {}",
            serde_json::to_string_pretty(&tx_receipt).unwrap()
        );
    }
}

async fn _get_encrypted_secret_key(
    contract_address: Address,
    client: Client,
    recipient: Address,
    messanger_contract: Option<
    BlockchainEmail<SignerMiddleware<Provider<Http>, Wallet<k256::ecdsa::SigningKey>>>,
    >,
) -> [u8; 129] {
    let contract = messanger_contract.unwrap_or_else(|| _init_contract(contract_address, &client));

    let encrypted_secret_key: Bytes = contract
        .get_encrypted_secret_key(recipient)
        .call()
        .await
        .unwrap();

    let encrypted_secret_key_array: [u8; 129] = encrypted_secret_key
        .to_vec()
        .try_into()
        .unwrap_or_else(|v: Vec<u8>| {
            panic!("Expected a Vec of length {} but it was {}", 129, v.len())
        });

    return encrypted_secret_key_array;
}

pub fn create_client(private_key: &str, rpc_url: &str) -> Client {
    let provider: Provider<Http> = Provider::<Http>::try_from(rpc_url).unwrap();
    let wallet: LocalWallet = init_wallet(private_key);
    let client = SignerMiddleware::new(provider.clone(), wallet.clone());

    return client;
}

pub async fn handle_send_message_command(mut send_message_args: crate::cli::SendMessageArgs) {
    send_message_args.validate_args().unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    });

    let contract_address_string = send_message_args.contract_address;
    let private_key_string = send_message_args.private_key;
    let recipient_string = send_message_args.recipient;
    let message_string = send_message_args.message;
    let rpc_url = send_message_args.rpc_url;
    let verbose = send_message_args.verbose;

    let contract_address: Address = contract_address_string.parse().unwrap();
    let recipient: Address = recipient_string.parse().unwrap();

    let client = create_client(&private_key_string, &rpc_url);

    _send_message(
        contract_address,
        &private_key_string,
        &client,
        recipient,
        &message_string,
        verbose,
    )
    .await;

    println!("Message sent!");
}

pub async fn handle_init_chat_command(mut init_chat_args: crate::cli::InitChatArgs) {
    init_chat_args.validate_args().unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    });

    let contract_address_string = init_chat_args.contract_address;
    let private_key_string = init_chat_args.private_key;
    let recipient_string = init_chat_args.recipient;
    let rpc_url = init_chat_args.rpc_url;
    let verbose = init_chat_args.verbose;

    let contract_address: Address = contract_address_string.parse().unwrap();
    let recipient: Address = recipient_string.parse().unwrap();

    let client = create_client(&private_key_string, &rpc_url);

    _add_encrypted_secret_keys(
        contract_address,
        &private_key_string,
        &client,
        recipient,
        verbose,
    )
    .await;

    println!("Chat initialized!");
}

pub async fn handle_add_public_key_command(mut add_public_key_args: crate::cli::AddPublicKeyArgs) {
    add_public_key_args.validate_args().unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    });

    let contract_address_string = add_public_key_args.contract_address;
    let private_key = add_public_key_args.private_key;
    let rpc_url = add_public_key_args.rpc_url;
    let verbose = add_public_key_args.verbose;

    let contract_address: Address = contract_address_string.parse().unwrap();

    let client = create_client(&private_key, &rpc_url);

    _add_public_key(contract_address, client, &private_key, verbose).await;

    println!("Public key added!");
}

pub async fn handle_get_messages_command(mut get_messages_args: crate::cli::GetMessagesArgs) {
    get_messages_args.validate_args().unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    });

    let contract_address_string = get_messages_args.contract_address;
    let private_key = get_messages_args.private_key;
    let recipient_string = get_messages_args.recipient;
    let rpc_url = get_messages_args.rpc_url;
    let verbose = get_messages_args.verbose;

    let contract_address: Address = contract_address_string.parse().unwrap();
    let recipient: Address = recipient_string.parse().unwrap();

    let client = create_client(&private_key, &rpc_url);

    let messages = _get_messages(contract_address, client, &private_key, recipient, verbose).await;

    for msg in &messages {
        println!("{}", msg);
    }
}

pub async fn handle_test_command() {
    println!("Generating test conversation... Please wait!");

    let contract_address: Address = _ADDR_CONTRACT_STR.parse().unwrap();
    let verbose = false;

    let sender: H160 = _ANVIL_ADDRESS_ACC_0.parse().unwrap();
    let recipient: H160 = _ANVIL_ADDRESS_ACC_1.parse().unwrap();

    let client_sender = create_client(&_ANVIL_PRIVATE_KEY_0, &_ANVIL_RPC_URL);
    let client_recipient = create_client(&_ANVIL_PRIVATE_KEY_1, &_ANVIL_RPC_URL);

    _add_encrypted_secret_keys(
        contract_address,
        &_ANVIL_PRIVATE_KEY_0,
        &client_sender,
        recipient,
        verbose,
    )
    .await;

    _send_message(contract_address, &_ANVIL_PRIVATE_KEY_0, &client_sender, recipient, "Hello!", verbose).await;
    _send_message(contract_address, &_ANVIL_PRIVATE_KEY_0, &client_sender, recipient, "This DAPP is amazing!!!", verbose).await;
    
    _send_message(contract_address, &_ANVIL_PRIVATE_KEY_1, &client_recipient, sender, "I love it!", verbose).await;
    
    _send_message(contract_address, &_ANVIL_PRIVATE_KEY_0, &client_sender, recipient, "I've had enough of Facebook Messenger. Privacy concerns and constant ads are driving me crazy!", verbose).await;
    _send_message(contract_address, &_ANVIL_PRIVATE_KEY_1, &client_recipient, sender, "Totally agree! I've been looking into decentralized messaging apps. Have you tried any?", verbose).await;


    _send_message(contract_address, &_ANVIL_PRIVATE_KEY_0, &client_sender, recipient, "Yes, I've switched to one. The blockchain technology behind it ensures end-to-end encryption and gives me full control over my data.", verbose).await;
    _send_message(contract_address, &_ANVIL_PRIVATE_KEY_1, &client_recipient, sender, "That's awesome! I love the idea of decentralized messaging. No more big corporations mining my data for targeted ads.", verbose).await;


    _send_message(contract_address, &_ANVIL_PRIVATE_KEY_0, &client_sender, recipient, "The best part is, I can communicate directly with friends without any middleman. It's fast and secure.", verbose).await;
    _send_message(contract_address, &_ANVIL_PRIVATE_KEY_1, &client_recipient, sender, "And there's no central authority tracking our every move. It's liberating!", verbose).await;

    _send_message(contract_address, &_ANVIL_PRIVATE_KEY_0, &client_sender, recipient, "Plus, the features enabled by blockchain, like decentralized file sharing and smart contracts, are game-changers.", verbose).await;
    _send_message(contract_address, &_ANVIL_PRIVATE_KEY_1, &client_recipient, sender, "I heard about that! Imagine having smart contract-based group chats. It's like a whole new level of interaction.", verbose).await;

    _send_message(contract_address, &_ANVIL_PRIVATE_KEY_0, &client_sender, recipient, "Exactly! I'm never going back to centralized messengers. Decentralized is the way forward!", verbose).await;
    _send_message(contract_address, &_ANVIL_PRIVATE_KEY_1, &client_recipient, sender, "Agreed. It's refreshing to be part of a community-driven platform rather than being just a product for a tech giant.", verbose).await;


    let messages = _get_messages(contract_address, client_sender, &_ANVIL_PRIVATE_KEY_0, recipient, verbose).await;

    for msg in &messages {
        println!("{}", msg);
    }

}
