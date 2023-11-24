// SPDX-License-Identifier: MIT
pragma solidity ^0.8.23;

import {EC} from "../src/EC.sol";
import {console} from "forge-std/console.sol";

/**
 * @title Blockchain email
 * @author Darko Panic
 * @notice Detailed info about dapp -> https://github.com/panicd9/Blockchain-messanger
 * @dev This contract is used for sending messages between users. Every user first needs to add his public key to the contract.
 * Every time chat is created between two users, CHAT SECRET KEY is generated and encrypted (on backend side) using public keys of both users.
 * CHAT SECRET KEY is used to avoid encrypting and storing every message 2 times (with sender and recipient public key).
 * Every message is encrypted using CHAT SECRET KEY and stored in contract. When user wants to read messages, he needs to decrypt them
 * using CHAT SECRET KEY, but he first needs to decrypt CHAT SECRET KEY with his own private key.
 *
 * Messages are stored in contract in following format: nonce (96 bits) + encryptedMessage + timestamp (256 bits).
 * Nonce is used to ensure that old communications cannot be reused in replay attacks.
 * Message is timestamped inside contract to avoid any manipulation with timestamps by users.
 *
 * In backend CLI tool i wrote in Rust, i used following libraries:
 * CHAT SECRET KEY is encrypted using ecies crate (Elliptic Curve Integrated Encryption Scheme).
 * Message is encrypted using aes_gcm_siv crate (AES-256-GCM).
 *
 * @dev For testing purposes, contract is initialized with first 5 users from Anvil testnet.
 * I used EC.sol smart contract from https://github.com/jbaylina/ecsol to generate public keys from nvil private test keys in Solidity.
 * Contract could be initialized from CLI tool, but i wanted to do it this way.
 *
 */
contract BlockchainEmail {
    struct PrivateKeyAndAddress {
        uint256 privateKey;
        address publicAddress;
    }

    struct PublicKey {
        bytes32 half1;
        bytes32 half2;
    }

    struct EncryptedSecretKeys {
        bytes lowerAddressSk;
        bytes higherAddressSk;
    }

    mapping(address user => PublicKey) public s_addressToPublicKey;
    mapping(address sender => mapping(address recipient => bytes[] messages)) public s_senderToRecipientMessages;
    mapping(bytes32 participants => EncryptedSecretKeys secretKeys) public s_participantsToEncryptedSk;

    event MessageSent(address indexed sender, address indexed recipient, bytes content, uint256 indexed timestamp);
    event PublicKeyAdded(address indexed user, bytes32 indexed publicKeyHalf1, bytes32 indexed publicKeyHalf2);

    error Messanger__SecretKeyAlreadyInitialized(bytes32 participant, address sender, address recipient);
    error Messanger__SecretKeyNotInitialized(bytes32 participant, address sender, address recipient);

    constructor() {
        PrivateKeyAndAddress[] memory privateKeysAndAddresses = new PrivateKeyAndAddress[](5);
        privateKeysAndAddresses[0].privateKey = 0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80;
        privateKeysAndAddresses[0].publicAddress = 0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266;
        privateKeysAndAddresses[1].privateKey = 0x59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d;
        privateKeysAndAddresses[1].publicAddress = 0x70997970C51812dc3A010C7d01b50e0d17dc79C8;
        privateKeysAndAddresses[2].privateKey = 0x5de4111afa1a4b94908f83103eb1f1706367c2e68ca870fc3fb9a804cdab365a;
        privateKeysAndAddresses[2].publicAddress = 0x3C44CdDdB6a900fa2b585dd299e03d12FA4293BC;
        privateKeysAndAddresses[3].privateKey = 0x7c852118294e51e653712a81e05800f419141751be58f605c371e15141b007a6;
        privateKeysAndAddresses[3].publicAddress = 0x90F79bf6EB2c4f870365E785982E1f101E93b906;
        privateKeysAndAddresses[4].privateKey = 0x47e179ec197488593b187f80a00eb0da91f1b9d0b13f8733639f19c30a34926a;
        privateKeysAndAddresses[4].publicAddress = 0x15d34AAf54267DB7D7c367839AAf71A00a2C6A65;

        EC ec = new EC();

        for (uint256 i = 0; i < privateKeysAndAddresses.length; i++) {
            (uint256 publicKeyHalf1, uint256 publicKeyHalf2) = ec.publicKey(privateKeysAndAddresses[i].privateKey);
            bytes32 publicKey1Bytes = bytes32(publicKeyHalf1);
            bytes32 publicKey2Bytes = bytes32(publicKeyHalf2);

            console.logBytes32(publicKey1Bytes);
            console.logBytes32(publicKey2Bytes);

            s_addressToPublicKey[privateKeysAndAddresses[i].publicAddress] =
                PublicKey({half1: publicKey1Bytes, half2: publicKey2Bytes});
        }
    }

    function sendMessage(address _recipient, bytes memory message) external {
        (bytes32 participants,,) = _getParticipantsHash(msg.sender, _recipient);

        if (!_isSecretKeyInitialized(participants)) {
            revert Messanger__SecretKeyNotInitialized(participants, msg.sender, _recipient);
        }

        bytes memory timestampBytes = abi.encodePacked(block.timestamp);
        message = abi.encodePacked(message, timestampBytes);

        s_senderToRecipientMessages[msg.sender][_recipient].push(message);
        emit MessageSent(msg.sender, _recipient, message, block.timestamp);
    }

    function addPublicKey(bytes32 publicKeyHalf1, bytes32 publicKeyHalf2) external {
        s_addressToPublicKey[msg.sender] = PublicKey({half1: publicKeyHalf1, half2: publicKeyHalf2});
        emit PublicKeyAdded(msg.sender, publicKeyHalf1, publicKeyHalf2);
    }

    function getPublicKey(address _userAddress) external view returns (bytes32, bytes32) {
        PublicKey memory publicKey = s_addressToPublicKey[_userAddress];
        return (publicKey.half1, publicKey.half2);
    }

    function getMessages(address _sender, address _recipient) external view returns (bytes[] memory) {
        return s_senderToRecipientMessages[_sender][_recipient];
    }

    function addEncryptedSecretKeys(
        address _recipient,
        bytes calldata senderEncryptedSecretKey,
        bytes calldata recipientEncryptedSecretKey
    ) external {
        (address lowerAddr, address higherAddr) =
            msg.sender < _recipient ? (msg.sender, _recipient) : (_recipient, msg.sender);
        bytes32 participants = keccak256(abi.encodePacked(lowerAddr, higherAddr));

        // Don't allow key reinitialization
        bool isSecretKeyInitialized = _isSecretKeyInitialized(participants);
        if (isSecretKeyInitialized) {
            revert Messanger__SecretKeyAlreadyInitialized(participants, msg.sender, _recipient);
        }

        if (msg.sender == lowerAddr) {
            s_participantsToEncryptedSk[participants].lowerAddressSk = senderEncryptedSecretKey;
            s_participantsToEncryptedSk[participants].higherAddressSk = recipientEncryptedSecretKey;
        } else {
            s_participantsToEncryptedSk[participants].lowerAddressSk = recipientEncryptedSecretKey;
            s_participantsToEncryptedSk[participants].higherAddressSk = senderEncryptedSecretKey;
        }
    }

    function getEncryptedSecretKey(address _recipient) external view returns (bytes memory) {
        (address lowerAddr, address higherAddr) =
            msg.sender < _recipient ? (msg.sender, _recipient) : (_recipient, msg.sender);
        bytes32 participants = keccak256(abi.encodePacked(lowerAddr, higherAddr));

        if (msg.sender == lowerAddr) {
            return s_participantsToEncryptedSk[participants].lowerAddressSk;
        } else {
            return s_participantsToEncryptedSk[participants].higherAddressSk;
        }
    }

    function _isSecretKeyInitialized(bytes32 participant) private view returns (bool) {
        EncryptedSecretKeys memory encryptedSecretKeys = s_participantsToEncryptedSk[participant];
        return encryptedSecretKeys.lowerAddressSk.length != 0 && encryptedSecretKeys.higherAddressSk.length != 0;
    }

    function _getParticipantsHash(address _sender, address _recipient)
        private
        pure
        returns (bytes32 participants, address lowerAddr, address higherAddr)
    {
        (lowerAddr, higherAddr) = _sender < _recipient ? (_sender, _recipient) : (_recipient, _sender);
        participants = keccak256(abi.encodePacked(lowerAddr, higherAddr));

        return (participants, lowerAddr, higherAddr);
    }
}
