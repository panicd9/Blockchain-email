# Blockchain Email

Author: Darko Panic

## Tools Used

This project was built using the following tools:

- **Rust**: The CLI tool responsible for cryptography and interaction with the smart contract.
- **Foundry**: Used for smart contract development and testing.
- **Solidity**: EVM smart contract.

## Example usage

1. `anvil`
2. `make deploy`
3. `cargo run -- test`

---

Alternatively:

1. `anvil`
2. `make deploy` in `smart-contracts` folder
3. `cargo run -- init-chat --recipient 0x70997970C51812dc3A010C7d01b50e0d17dc79C8`
4. `cargo run -- send-message -r 0x70997970C51812dc3A010C7d01b50e0d17dc79C8 -m  "HELLO!"`
5. `cargo run -- get-messages -r 0x70997970C51812dc3A010C7d01b50e0d17dc79C8`

## About

This DAPP is used for sending messages between users on a blockchain. Every user first needs to add their public key to the contract. Every time a chat is created between two users, a CHAT SECRET KEY is generated and encrypted (on the backend side) using the public keys of both users.

The CHAT SECRET KEY is used to avoid encrypting and storing every message twice (with the sender's and recipient's public key). Every message is encrypted using the CHAT SECRET KEY and stored in the contract. When a user wants to read messages, they need to decrypt them using the CHAT SECRET KEY, but they first need to decrypt the CHAT SECRET KEY with their own private key.

## Message Representation

Messages are stored in the contract in the following format: nonce (96 bits) + encryptedMessage + timestamp (256 bits). 

- The nonce is a random number that is used only once. It is used to ensure that old communications cannot be reused in replay attacks. 
- The encryptedMessage is the actual content of the message, encrypted using the CHAT SECRET KEY.
- The timestamp is the time when the message was sent, recorded inside the contract to avoid any manipulation with timestamps by users.

## CLI tool

### Commands:

- **add-public-key**:  Add public key to the contract
  - **Usage: blockchain-email add-public-key [OPTIONS]**

        Options:
            -c, --contract-address <CONTRACT_ADDRESS>
                    [default: 0x5FbDB2315678afecb367f032d93F642f64180aa3]
            -p, --private-key <PRIVATE_KEY>
                    [default: 0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80]
            -u, --rpc-url <RPC_URL>
                    [default: http://127.0.0.1:8545]
            -v, --verbose
                    
            -h, --help

- **init-chat**:       Initialize a chat - generate secret key, encrypt it with sender's and recipient's public keys and send it to the contract
  - **Usage: blockchain-email init-chat [OPTIONS] --recipient <RECIPIENT>**

        Options:
            -c, --contract-address <CONTRACT_ADDRESS>
                    [default: 0x5FbDB2315678afecb367f032d93F642f64180aa3]
            -p, --private-key <PRIVATE_KEY>
                    [default: 0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80]
            -r, --recipient <RECIPIENT>
                    
            -u, --rpc-url <RPC_URL>
                    [default: http://127.0.0.1:8545]
            -v, --verbose
                    
            -h, --help
                    Print help
- **send-message**:    Send a message
  - **Usage: blockchain-email send-message [OPTIONS] --recipient <RECIPIENT> --message <MESSAGE>**

        Options:
            -c, --contract-address <CONTRACT_ADDRESS>
                    [default: 0x5FbDB2315678afecb367f032d93F642f64180aa3]
            -p, --private-key <PRIVATE_KEY>
                    [default: 0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80]
            -r, --recipient <RECIPIENT>
                    
            -m, --message <MESSAGE>
                    
            -u, --rpc-url <RPC_URL>
                    [default: http://127.0.0.1:8545]
            -v, --verbose
                    
            -h, --help
                    Print help
- **get-messages**:    Get messages
  - Usage: blockchain-email get-messages [OPTIONS] --recipient <RECIPIENT>

        Options:
            -c, --contract-address <CONTRACT_ADDRESS>
                    [default: 0x5FbDB2315678afecb367f032d93F642f64180aa3]
            -p, --private-key <PRIVATE_KEY>
                    [default: 0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80]
            -u, --rpc-url <RPC_URL>
                    [default: http://127.0.0.1:8545]
            -r, --recipient <RECIPIENT>
                    
            -v, --verbose
                    
            -h, --help
                    Print help
- **help**:            Print this message or the help of the given subcommand(s)
  
### The following libraries are used:

- The CHAT SECRET KEY is encrypted using the `ecies` crate (Elliptic Curve Integrated Encryption Scheme).
- The message is encrypted using the `aes_gcm_siv` crate (AES-256-GCM).

### Testing

For testing purposes, the contract is initialized with the first 5 users from the Anvil testnet. The EC.sol smart contract from [jBaylina's ecsol](https://github.com/jbaylina/ecsol) was used to generate public keys from Anvil private test keys in Solidity. The contract could be initialized from the CLI tool, but I chose to do it this way.

#### Available Accounts

1. 0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266
2. 0x70997970C51812dc3A010C7d01b50e0d17dc79C8
3. 0x3C44CdDdB6a900fa2b585dd299e03d12FA4293BC
4. 0x90F79bf6EB2c4f870365E785982E1f101E93b906
5. 0x15d34AAf54267DB7D7c367839AAf71A00a2C6A65


#### Private Keys

1. 0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80
2. 0x59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d
3. 0x5de4111afa1a4b94908f83103eb1f1706367c2e68ca870fc3fb9a804cdab365a
4. 0x7c852118294e51e653712a81e05800f419141751be58f605c371e15141b007a6
5. 0x47e179ec197488593b187f80a00eb0da91f1b9d0b13f8733639f19c30a34926a