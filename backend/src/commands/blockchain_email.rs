pub use blockchain_email::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types,
)]
pub mod blockchain_email {
    const _: () = {
        ::core::include_bytes!(
            "BlockchainEmail.json",
        );
    };
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("addEncryptedSecretKeys"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "addEncryptedSecretKeys",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_recipient"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "senderEncryptedSecretKey",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "recipientEncryptedSecretKey",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("addPublicKey"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("addPublicKey"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("publicKeyHalf1"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("publicKeyHalf2"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getEncryptedSecretKey"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getEncryptedSecretKey",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_recipient"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getMessages"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getMessages"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_sender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_recipient"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getPublicKey"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getPublicKey"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_userAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("s_addressToPublicKey"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "s_addressToPublicKey",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("user"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("half1"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("half2"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("s_participantsToEncryptedSk"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "s_participantsToEncryptedSk",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("participants"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("lowerAddressSk"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("higherAddressSk"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("s_senderToRecipientMessages"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "s_senderToRecipientMessages",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("sender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("recipient"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("messages"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("sendMessage"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("sendMessage"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_recipient"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("message"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("MessageSent"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("MessageSent"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("sender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("recipient"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("content"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("timestamp"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("PublicKeyAdded"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("PublicKeyAdded"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("user"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("publicKeyHalf1"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("publicKeyHalf2"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
            ]),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned(
                        "Messanger__SecretKeyAlreadyInitialized",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "Messanger__SecretKeyAlreadyInitialized",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("participant"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("sender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("recipient"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "Messanger__SecretKeyNotInitialized",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "Messanger__SecretKeyNotInitialized",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("participant"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("sender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("recipient"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
            ]),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static BLOCKCHAINEMAIL_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15b\0\0\x11W`\0\x80\xFD[P`@\x80Q`\x05\x80\x82R`\xC0\x82\x01\x90\x92R`\0\x91\x81` \x01[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x81R` \x01\x90`\x01\x90\x03\x90\x81b\0\0*W\x90PP\x90P\x7F\xAC\tt\xBE\xC3\x9A\x17\xE3k\xA4\xA6\xB4\xD28\xFF\x94K\xAC\xB4x\xCB\xED^\xFC\xAExM{\xF4\xF2\xFF\x80\x81`\0\x81Q\x81\x10b\0\0\x8BWb\0\0\x8Bb\0\x05)V[` \x02` \x01\x01Q`\0\x01\x81\x81RPPs\xF3\x9F\xD6\xE5\x1A\xAD\x88\xF6\xF4\xCEj\xB8\x82ry\xCF\xFF\xB9\"f\x81`\0\x81Q\x81\x10b\0\0\xC6Wb\0\0\xC6b\0\x05)V[` \x02` \x01\x01Q` \x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x7FY\xC6\x99^\x99\x8F\x97\xA5\xA0\x04If\xF0\x94S\x89\xDC\x9E\x86\xDA\xE8\x8Cz\x84\x12\xF4`;kxi\r\x81`\x01\x81Q\x81\x10b\0\x01!Wb\0\x01!b\0\x05)V[` \x02` \x01\x01Q`\0\x01\x81\x81RPPsp\x99yp\xC5\x18\x12\xDC:\x01\x0C}\x01\xB5\x0E\r\x17\xDCy\xC8\x81`\x01\x81Q\x81\x10b\0\x01\\Wb\0\x01\\b\0\x05)V[` \x02` \x01\x01Q` \x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x7F]\xE4\x11\x1A\xFA\x1AK\x94\x90\x8F\x83\x10>\xB1\xF1pcg\xC2\xE6\x8C\xA8p\xFC?\xB9\xA8\x04\xCD\xAB6Z\x81`\x02\x81Q\x81\x10b\0\x01\xB7Wb\0\x01\xB7b\0\x05)V[` \x02` \x01\x01Q`\0\x01\x81\x81RPPs<D\xCD\xDD\xB6\xA9\0\xFA+X]\xD2\x99\xE0=\x12\xFAB\x93\xBC\x81`\x02\x81Q\x81\x10b\0\x01\xF2Wb\0\x01\xF2b\0\x05)V[` \x02` \x01\x01Q` \x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x7F|\x85!\x18)NQ\xE6Sq*\x81\xE0X\0\xF4\x19\x14\x17Q\xBEX\xF6\x05\xC3q\xE1QA\xB0\x07\xA6\x81`\x03\x81Q\x81\x10b\0\x02MWb\0\x02Mb\0\x05)V[` \x02` \x01\x01Q`\0\x01\x81\x81RPPs\x90\xF7\x9B\xF6\xEB,O\x87\x03e\xE7\x85\x98.\x1F\x10\x1E\x93\xB9\x06\x81`\x03\x81Q\x81\x10b\0\x02\x88Wb\0\x02\x88b\0\x05)V[` \x02` \x01\x01Q` \x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x7FG\xE1y\xEC\x19t\x88Y;\x18\x7F\x80\xA0\x0E\xB0\xDA\x91\xF1\xB9\xD0\xB1?\x873c\x9F\x19\xC3\n4\x92j\x81`\x04\x81Q\x81\x10b\0\x02\xE3Wb\0\x02\xE3b\0\x05)V[` \x02` \x01\x01Q`\0\x01\x81\x81RPPs\x15\xD3J\xAFT&}\xB7\xD7\xC3g\x83\x9A\xAFq\xA0\n,je\x81`\x04\x81Q\x81\x10b\0\x03\x1EWb\0\x03\x1Eb\0\x05)V[` \x02` \x01\x01Q` \x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP`\0`@Qb\0\x03R\x90b\0\x05\x1BV[`@Q\x80\x91\x03\x90`\0\xF0\x80\x15\x80\x15b\0\x03oW=`\0\x80>=`\0\xFD[P\x90P`\0[\x82Q\x81\x10\x15b\0\x04\xA2W`\0\x80\x83`\x01`\x01`\xA0\x1B\x03\x16c\x89@\xAE\xBE\x86\x85\x81Q\x81\x10b\0\x03\xA6Wb\0\x03\xA6b\0\x05)V[` \x02` \x01\x01Q`\0\x01Q`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01b\0\x03\xD1\x91\x81R` \x01\x90V[`@\x80Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15b\0\x03\xEEW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\x04\x14\x91\x90b\0\x05?V[\x90\x92P\x90P\x81\x81b\0\x04&\x82b\0\x04\xABV[b\0\x041\x81b\0\x04\xABV[`@Q\x80`@\x01`@R\x80\x83\x81R` \x01\x82\x81RP`\0\x80\x89\x88\x81Q\x81\x10b\0\x04^Wb\0\x04^b\0\x05)V[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Q\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x82R\x81\x81\x01\x92\x90\x92R`@\x01`\0 \x82Q\x81U\x91\x01Q`\x01\x91\x82\x01U\x94\x90\x94\x01\x93Pb\0\x03u\x92PPPV[PPPb\0\x05dV[b\0\x04\xF7\x81`@Q`$\x01b\0\x04\xC3\x91\x81R` \x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x90\x81\x16c'\xB7\xCF\x85`\xE0\x1B\x17\x90\x91Rb\0\x04\xFA\x16V[PV[\x80Qjconsole.log` \x83\x01`\0\x80\x84\x83\x85Z\xFAPPPPPV[a\x07-\x80b\0\x15\xC8\x839\x01\x90V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0\x80`@\x83\x85\x03\x12\x15b\0\x05SW`\0\x80\xFD[PP\x80Q` \x90\x91\x01Q\x90\x92\x90\x91PV[a\x10T\x80b\0\x05t`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\x93W`\x005`\xE0\x1C\x80cxp\xFBJ\x11a\0fW\x80cxp\xFBJ\x14a\x013W\x80c\x81\x0B\x91x\x14a\x01FW\x80c\x85|\xDB\xB8\x14a\x01YW\x80c\xAA>\xF8\x0E\x14a\x01\x9DW\x80c\xBB]\xDB\x0F\x14a\x01\xBDW`\0\x80\xFD[\x80c\x11\xD5\x1E1\x14a\0\x98W\x80c\x15\xCB?\xFB\x14a\0\xC2W\x80c8\x82\xB5]\x14a\0\xFEW\x80c@.\x0FS\x14a\x01\x13W[`\0\x80\xFD[a\0\xABa\0\xA66`\x04a\nDV[a\x01\xD0V[`@Qa\0\xB9\x92\x91\x90a\n\xADV[`@Q\x80\x91\x03\x90\xF3[a\0\xE9a\0\xD06`\x04a\n\xF7V[`\0` \x81\x90R\x90\x81R`@\x90 \x80T`\x01\x90\x91\x01T\x82V[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01a\0\xB9V[a\x01\x11a\x01\x0C6`\x04a\x0B[V[a\x02\xFCV[\0[a\x01&a\x01!6`\x04a\x0B\xDCV[a\x04\"V[`@Qa\0\xB9\x91\x90a\x0C\x18V[a\x01&a\x01A6`\x04a\n\xF7V[a\x04\xE8V[a\x01\x11a\x01T6`\x04a\x0C+V[a\x06\x0EV[a\0\xE9a\x01g6`\x04a\n\xF7V[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R` \x81\x81R`@\x91\x82\x90 \x82Q\x80\x84\x01\x90\x93R\x80T\x80\x84R`\x01\x90\x91\x01T\x92\x90\x91\x01\x82\x90R\x91V[a\x01\xB0a\x01\xAB6`\x04a\x0CMV[a\x06mV[`@Qa\0\xB9\x91\x90a\x0C\x80V[a\x01\x11a\x01\xCB6`\x04a\x0C\xFAV[a\x07iV[`\x02` R`\0\x90\x81R`@\x90 \x80T\x81\x90a\x01\xEB\x90a\r\xBCV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x02\x17\x90a\r\xBCV[\x80\x15a\x02dW\x80`\x1F\x10a\x029Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x02dV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x02GW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90\x80`\x01\x01\x80Ta\x02y\x90a\r\xBCV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x02\xA5\x90a\r\xBCV[\x80\x15a\x02\xF2W\x80`\x1F\x10a\x02\xC7Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x02\xF2V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x02\xD5W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x82V[`\0\x80`\x01`\x01`\xA0\x1B\x03\x87\x163\x10a\x03\x16W\x863a\x03\x19V[3\x87[\x91P\x91P`\0\x82\x82`@Q` \x01a\x032\x92\x91\x90a\r\xF6V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P`\0a\x03U\x82a\x08\x87V[\x90P\x80\x15a\x03\x93W`@Qc\xC3p\x1A\xC9`\xE0\x1B\x81R`\x04\x81\x01\x83\x90R3`$\x82\x01R`\x01`\x01`\xA0\x1B\x03\x8A\x16`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x84\x163\x03a\x03\xDFW`\0\x82\x81R`\x02` R`@\x90 a\x03\xBC\x88\x8A\x83a\x0EnV[P`\0\x82\x81R`\x02` R`@\x90 `\x01\x01a\x03\xD9\x86\x88\x83a\x0EnV[Pa\x04\x17V[`\0\x82\x81R`\x02` R`@\x90 a\x03\xF8\x86\x88\x83a\x0EnV[P`\0\x82\x81R`\x02` R`@\x90 `\x01\x01a\x04\x15\x88\x8A\x83a\x0EnV[P[PPPPPPPPPV[`\x01` R\x82`\0R`@`\0 ` R\x81`\0R`@`\0 \x81\x81T\x81\x10a\x04JW`\0\x80\xFD[\x90`\0R` `\0 \x01`\0\x92P\x92PPP\x80Ta\x04g\x90a\r\xBCV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x04\x93\x90a\r\xBCV[\x80\x15a\x04\xE0W\x80`\x1F\x10a\x04\xB5Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x04\xE0V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x04\xC3W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81V[```\0\x80`\x01`\x01`\xA0\x1B\x03\x84\x163\x10a\x05\x04W\x833a\x05\x07V[3\x84[\x91P\x91P`\0\x82\x82`@Q` \x01a\x05 \x92\x91\x90a\r\xF6V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x82`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x03a\x05\xF2W`\0\x81\x81R`\x02` R`@\x90 \x80Ta\x05j\x90a\r\xBCV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x05\x96\x90a\r\xBCV[\x80\x15a\x05\xE3W\x80`\x1F\x10a\x05\xB8Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x05\xE3V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x05\xC6W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x93PPPP\x91\x90PV[`\0\x81\x81R`\x02` R`@\x90 `\x01\x01\x80Ta\x05j\x90a\r\xBCV[`@\x80Q\x80\x82\x01\x82R\x83\x81R` \x80\x82\x01\x84\x81R3`\0\x81\x81R\x92\x83\x90R\x84\x83 \x93Q\x84U\x90Q`\x01\x90\x93\x01\x92\x90\x92U\x91Q\x83\x92\x85\x92\x91\x7F\xC2\xA3A\xA2\xBB\x1D\xCD`x@Y\xFB\xC0\xDE\xD5,O\x1F\x80Y\r\x86\xFA\xDE+v\x18w!\x88\xB7\xFA\x91\x90\xA4PPV[`\x01`\x01`\xA0\x1B\x03\x80\x83\x16`\0\x90\x81R`\x01` \x90\x81R`@\x80\x83 \x93\x85\x16\x83R\x92\x81R\x82\x82 \x80T\x84Q\x81\x84\x02\x81\x01\x84\x01\x90\x95R\x80\x85R``\x94\x93\x91\x92\x90\x91\x90\x84\x01[\x82\x82\x10\x15a\x07]W\x83\x82\x90`\0R` `\0 \x01\x80Ta\x06\xD0\x90a\r\xBCV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x06\xFC\x90a\r\xBCV[\x80\x15a\x07IW\x80`\x1F\x10a\x07\x1EWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x07IV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x07,W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\x06\xB1V[PPPP\x90P\x92\x91PPV[`\0a\x07u3\x84a\t\xE5V[PP\x90Pa\x07\x82\x81a\x08\x87V[a\x07\xB7W`@Qc/0\"\x17`\xE2\x1B\x81R`\x04\x81\x01\x82\x90R3`$\x82\x01R`\x01`\x01`\xA0\x1B\x03\x84\x16`D\x82\x01R`d\x01a\x03\x8AV[`@\x80QB` \x82\x01R`\0\x91\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x82\x81`@Q` \x01a\x07\xE9\x92\x91\x90a\x0F/V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x81R3`\0\x90\x81R`\x01` \x81\x81R\x83\x83 `\x01`\x01`\xA0\x1B\x03\x8A\x16\x84R\x81R\x92\x82 \x80T\x91\x82\x01\x81U\x82R\x91\x90 \x91\x94P\x01a\x084\x84\x82a\x0F^V[PB\x84`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x7F\xFA\x9D \x8F\x16\xE4\xEB_\xE2\xB7\xDA\xCC\x89\xDC\x18\xB1,\x13\xD9\x16^elU\x1C\x83D\x1E\xAD\xF5\xB1\xA2\x86`@Qa\x08y\x91\x90a\x0C\x18V[`@Q\x80\x91\x03\x90\xA4PPPPV[`\0\x81\x81R`\x02` R`@\x80\x82 \x81Q\x80\x83\x01\x90\x92R\x80T\x83\x92\x91\x90\x82\x90\x82\x90a\x08\xB1\x90a\r\xBCV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x08\xDD\x90a\r\xBCV[\x80\x15a\t*W\x80`\x1F\x10a\x08\xFFWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\t*V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\t\rW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x01\x82\x01\x80Ta\tC\x90a\r\xBCV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\to\x90a\r\xBCV[\x80\x15a\t\xBCW\x80`\x1F\x10a\t\x91Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\t\xBCV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\t\x9FW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPP\x91\x90\x92RPP\x81QQ\x91\x92PP\x15\x80\x15\x90a\t\xDEWP` \x81\x01QQ\x15\x15[\x93\x92PPPV[`\0\x80`\0\x83`\x01`\x01`\xA0\x1B\x03\x16\x85`\x01`\x01`\xA0\x1B\x03\x16\x10a\n\nW\x83\x85a\n\rV[\x84\x84[`@Q\x91\x93P\x91Pa\n%\x90\x83\x90\x83\x90` \x01a\r\xF6V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x92P\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15a\nVW`\0\x80\xFD[P5\x91\x90PV[`\0[\x83\x81\x10\x15a\nxW\x81\x81\x01Q\x83\x82\x01R` \x01a\n`V[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra\n\x99\x81` \x86\x01` \x86\x01a\n]V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[`@\x81R`\0a\n\xC0`@\x83\x01\x85a\n\x81V[\x82\x81\x03` \x84\x01Ra\n\xD2\x81\x85a\n\x81V[\x95\x94PPPPPV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\n\xF2W`\0\x80\xFD[\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x0B\tW`\0\x80\xFD[a\t\xDE\x82a\n\xDBV[`\0\x80\x83`\x1F\x84\x01\x12a\x0B$W`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0B<W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\x0BTW`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80`\0\x80`\0``\x86\x88\x03\x12\x15a\x0BsW`\0\x80\xFD[a\x0B|\x86a\n\xDBV[\x94P` \x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x0B\x99W`\0\x80\xFD[a\x0B\xA5\x89\x83\x8A\x01a\x0B\x12V[\x90\x96P\x94P`@\x88\x015\x91P\x80\x82\x11\x15a\x0B\xBEW`\0\x80\xFD[Pa\x0B\xCB\x88\x82\x89\x01a\x0B\x12V[\x96\x99\x95\x98P\x93\x96P\x92\x94\x93\x92PPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x0B\xF1W`\0\x80\xFD[a\x0B\xFA\x84a\n\xDBV[\x92Pa\x0C\x08` \x85\x01a\n\xDBV[\x91P`@\x84\x015\x90P\x92P\x92P\x92V[` \x81R`\0a\t\xDE` \x83\x01\x84a\n\x81V[`\0\x80`@\x83\x85\x03\x12\x15a\x0C>W`\0\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[`\0\x80`@\x83\x85\x03\x12\x15a\x0C`W`\0\x80\xFD[a\x0Ci\x83a\n\xDBV[\x91Pa\x0Cw` \x84\x01a\n\xDBV[\x90P\x92P\x92\x90PV[`\0` \x80\x83\x01` \x84R\x80\x85Q\x80\x83R`@\x86\x01\x91P`@\x81`\x05\x1B\x87\x01\x01\x92P` \x87\x01`\0[\x82\x81\x10\x15a\x0C\xD7W`?\x19\x88\x86\x03\x01\x84Ra\x0C\xC5\x85\x83Qa\n\x81V[\x94P\x92\x85\x01\x92\x90\x85\x01\x90`\x01\x01a\x0C\xA9V[P\x92\x97\x96PPPPPPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0\x80`@\x83\x85\x03\x12\x15a\r\rW`\0\x80\xFD[a\r\x16\x83a\n\xDBV[\x91P` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\r3W`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\rGW`\0\x80\xFD[\x815\x81\x81\x11\x15a\rYWa\rYa\x0C\xE4V[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x83\x82\x11\x81\x83\x10\x17\x15a\r\x81Wa\r\x81a\x0C\xE4V[\x81`@R\x82\x81R\x88` \x84\x87\x01\x01\x11\x15a\r\x9AW`\0\x80\xFD[\x82` \x86\x01` \x83\x017`\0` \x84\x83\x01\x01R\x80\x95PPPPPP\x92P\x92\x90PV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\r\xD0W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\r\xF0WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19``\x93\x84\x1B\x81\x16\x82R\x91\x90\x92\x1B\x16`\x14\x82\x01R`(\x01\x90V[`\x1F\x82\x11\x15a\x0EiW`\0\x81`\0R` `\0 `\x1F\x85\x01`\x05\x1C\x81\x01` \x86\x10\x15a\x0EFWP\x80[`\x1F\x85\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\x0EeW\x82\x81U`\x01\x01a\x0ERV[PPP[PPPV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11\x15a\x0E\x86Wa\x0E\x86a\x0C\xE4V[a\x0E\x9A\x83a\x0E\x94\x83Ta\r\xBCV[\x83a\x0E\x1DV[`\0`\x1F\x84\x11`\x01\x81\x14a\x0E\xCEW`\0\x85\x15a\x0E\xB6WP\x83\x82\x015[`\0\x19`\x03\x87\x90\x1B\x1C\x19\x16`\x01\x86\x90\x1B\x17\x83Ua\x0F(V[`\0\x83\x81R` \x90 `\x1F\x19\x86\x16\x90\x83[\x82\x81\x10\x15a\x0E\xFFW\x86\x85\x015\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01a\x0E\xDFV[P\x86\x82\x10\x15a\x0F\x1CW`\0\x19`\xF8\x88`\x03\x1B\x16\x1C\x19\x84\x87\x015\x16\x81U[PP`\x01\x85`\x01\x1B\x01\x83U[PPPPPV[`\0\x83Qa\x0FA\x81\x84` \x88\x01a\n]V[\x83Q\x90\x83\x01\x90a\x0FU\x81\x83` \x88\x01a\n]V[\x01\x94\x93PPPPV[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0FxWa\x0Fxa\x0C\xE4V[a\x0F\x8C\x81a\x0F\x86\x84Ta\r\xBCV[\x84a\x0E\x1DV[` \x80`\x1F\x83\x11`\x01\x81\x14a\x0F\xC1W`\0\x84\x15a\x0F\xA9WP\x85\x83\x01Q[`\0\x19`\x03\x86\x90\x1B\x1C\x19\x16`\x01\x85\x90\x1B\x17\x85Ua\x0EeV[`\0\x85\x81R` \x81 `\x1F\x19\x86\x16\x91[\x82\x81\x10\x15a\x0F\xF0W\x88\x86\x01Q\x82U\x94\x84\x01\x94`\x01\x90\x91\x01\x90\x84\x01a\x0F\xD1V[P\x85\x82\x10\x15a\x10\x0EW\x87\x85\x01Q`\0\x19`\x03\x88\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPPP`\x01\x90\x81\x1B\x01\x90UPV\xFE\xA2dipfsX\"\x12 O\0\xAC\xEC>\x10\xB0\xAD\x10\xCFZ\xC1\xCCz,\x91_\x0E\xE3\\\xEA\x9A#\x83\xC6\xC7\x9D\x01 \xA5\xC8\xD2dsolcC\0\x08\x17\x003`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[Pa\x07\r\x80a\0 `\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\x9EW`\x005`\xE0\x1C\x80c\x8C\xEC\xF6n\x11a\0fW\x80c\x8C\xEC\xF6n\x14a\x01\x1CW\x80c\x91?BL\x14a\x01=W\x80c\xDB1\x883\x14a\x01kW\x80c\xE2A\xC1\xD9\x14a\x01~W\x80c\xF4r\x89\xE1\x14a\x01\x91W`\0\x80\xFD[\x80c\x018\xE3\x1B\x14a\0\xA3W\x80c\x1E\xCF\xE6M\x14a\0\xD0W\x80c[vH\x11\x14a\0\xE3W\x80c_\x97-\xF8\x14a\0\xF6W\x80c\x89@\xAE\xBE\x14a\x01\tW[`\0\x80\xFD[a\0\xB6a\0\xB16`\x04a\x05\xB5V[a\x01\xA4V[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01[`@Q\x80\x91\x03\x90\xF3[a\0\xB6a\0\xDE6`\x04a\x05\xB5V[a\x01\xD6V[a\0\xB6a\0\xF16`\x04a\x05\xB5V[a\x01\xFFV[a\0\xB6a\x01\x046`\x04a\x05\xB5V[a\x02\x18V[a\0\xB6a\x01\x176`\x04a\x05\xE7V[a\x02<V[a\x01/a\x01*6`\x04a\x05\xE7V[a\x02\xC3V[`@Q\x90\x81R` \x01a\0\xC7V[a\x01Pa\x01K6`\x04a\x05\xB5V[a\x031V[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R``\x01a\0\xC7V[a\x01Pa\x01y6`\x04a\x06\0V[a\x03\xBAV[a\0\xB6a\x01\x8C6`\x04a\x06CV[a\x05IV[a\x01Pa\x01\x9F6`\x04a\x06CV[a\x05\x93V[`\0\x80d\x01\0\0\x03\xD0\x19\x80\x86\x86\td\x01\0\0\x03\xD0\x19[\x88\x86\t\x08d\x01\0\0\x03\xD0\x19[\x84\x87\t\x90\x97\x90\x96P\x94PPPPPV[`\0\x80d\x01\0\0\x03\xD0\x19\x80\x86a\x01\xF2\x87d\x01\0\0\x03\xD0\x19a\x06\x85V[\td\x01\0\0\x03\xD0\x19a\x01\xBAV[`\0\x80d\x01\0\0\x03\xD0\x19\x84\x87\td\x01\0\0\x03\xD0\x19a\x01\xC6V[`\0\x80d\x01\0\0\x03\xD0\x19\x83\x87\td\x01\0\0\x03\xD0\x19\x85\x87\t\x90\x97\x90\x96P\x94PPPPPV[`\0\x80`\0\x80`\0a\x02\x91\x86\x7Fy\xBEf~\xF9\xDC\xBB\xACU\xA0b\x95\xCE\x87\x0B\x07\x02\x9B\xFC\xDB-\xCE(\xD9Y\xF2\x81[\x16\xF8\x17\x98\x7FH:\xDAw&\xA3\xC4e]\xA4\xFB\xFC\x0E\x11\x08\xA8\xFD\x17\xB4H\xA6\x85T\x19\x9CG\xD0\x8F\xFB\x10\xD4\xB8`\x01a\x031V[\x91\x94P\x92P\x90Pa\x02\xA1\x81a\x02\xC3V[\x90Pd\x01\0\0\x03\xD0\x19\x81\x84\t\x94Pd\x01\0\0\x03\xD0\x19\x81\x83\t\x93PPPP\x91P\x91V[`\0\x80`\x01d\x01\0\0\x03\xD0\x19\x84\x83[\x81\x15a\x03&Wa\x02\xE2\x82\x84a\x06\x9EV[\x90P\x83d\x01\0\0\x03\xD0\x19\x80\x86\x84\ta\x03\0\x90d\x01\0\0\x03\xD0\x19a\x06\x85V[\x87\x08\x90\x95P\x93P\x81a\x03\x12\x81\x83a\x06\xC0V[a\x03\x1C\x90\x85a\x06\x85V[\x90\x93P\x91Pa\x02\xD2V[P\x92\x95\x94PPPPPV[`\0\x80\x80\x86\x86\x86\x86\x84\x80`\x01\x86\x82\x03a\x03[W`\0\x80`\x01\x99P\x99P\x99PPPPPPPPa\x03\xB0V[\x86\x15a\x03\xA4W`\x01\x87\x16\x15a\x03\x80Wa\x03x\x83\x83\x83\x89\x89\x89a\x03\xBAV[\x91\x94P\x92P\x90P[a\x03\x8B`\x02\x88a\x06\x9EV[\x96Pa\x03\x98\x86\x86\x86a\x05\x93V[\x91\x97P\x95P\x93Pa\x03[V[\x91\x98P\x96P\x94PPPPP[\x94P\x94P\x94\x91PPV[`\0\x80\x80\x80\x80\x80\x80\x8C\x15\x80\x15a\x03\xCEWP\x8B\x15[\x15a\x03\xE5W\x89\x89\x89\x96P\x96P\x96PPPPPa\x05=V[\x89\x15\x80\x15a\x03\xF1WP\x88\x15[\x15a\x04\x08W\x8C\x8C\x8C\x96P\x96P\x96PPPPPa\x05=V[\x89\x8D\x14\x80\x15a\x04\x16WP\x88\x8C\x14[\x15a\x04jWa\x04'\x8D\x8C\x8F\x8Ea\x01\xFFV[\x90\x94P\x92Pa\x04:\x84\x84`\x03`\x01a\x01\xFFV[\x90\x94P\x92Pa\x04M\x84\x84`\0`\x01a\x01\xA4V[\x90\x94P\x92Pa\x04`\x8C\x8C`\x02`\x01a\x01\xFFV[\x90\x92P\x90Pa\x04\x8DV[a\x04v\x89\x89\x8E\x8Ea\x01\xD6V[\x90\x94P\x92Pa\x04\x87\x8A\x89\x8F\x8Ea\x01\xD6V[\x90\x92P\x90P[a\x04\x99\x84\x84\x84\x84a\x02\x18V[\x90\x94P\x92Pa\x04\xAA\x84\x84\x81\x81a\x01\xFFV[\x90\x97P\x91Pa\x04\xBB\x87\x83\x8F\x8Ea\x01\xD6V[\x90\x97P\x91Pa\x04\xCC\x87\x83\x8C\x8Ba\x01\xD6V[\x90\x97P\x91Pa\x04\xDD\x8D\x8C\x89\x85a\x01\xD6V[\x90\x96P\x90Pa\x04\xEE\x86\x82\x86\x86a\x01\xFFV[\x90\x96P\x90Pa\x04\xFF\x86\x82\x8E\x8Ea\x01\xD6V[\x90\x96P\x90P\x81\x81\x14a\x054Wd\x01\0\0\x03\xD0\x19\x81\x88\t\x96Pd\x01\0\0\x03\xD0\x19\x82\x87\t\x95Pd\x01\0\0\x03\xD0\x19\x81\x83\t\x94Pa\x058V[\x81\x94P[PPPP[\x96P\x96P\x96\x93PPPPV[`\0\x80`\0\x80`\0a\x05^\x88\x88\x88`\x01a\x031V[\x91\x94P\x92P\x90Pa\x05n\x81a\x02\xC3V[\x90Pd\x01\0\0\x03\xD0\x19\x81\x84\t\x94Pd\x01\0\0\x03\xD0\x19\x81\x83\t\x93PPPP\x93P\x93\x91PPV[`\0\x80`\0a\x05\xA6\x86\x86\x86\x89\x89\x89a\x03\xBAV[\x91\x98\x90\x97P\x90\x95P\x93PPPPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x05\xCBW`\0\x80\xFD[PP\x825\x94` \x84\x015\x94P`@\x84\x015\x93``\x015\x92P\x90PV[`\0` \x82\x84\x03\x12\x15a\x05\xF9W`\0\x80\xFD[P5\x91\x90PV[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15a\x06\x19W`\0\x80\xFD[PP\x845\x96` \x86\x015\x96P`@\x86\x015\x95``\x81\x015\x95P`\x80\x81\x015\x94P`\xA0\x015\x92P\x90PV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x06XW`\0\x80\xFD[PP\x815\x93` \x83\x015\x93P`@\x90\x92\x015\x91\x90PV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x81\x81\x03\x81\x81\x11\x15a\x06\x98Wa\x06\x98a\x06oV[\x92\x91PPV[`\0\x82a\x06\xBBWcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x06\x98Wa\x06\x98a\x06oV\xFE\xA2dipfsX\"\x12 \x13t}\0\x995\xF6\xDAmn\xE7k\xD5\xF3\xBC\xAC\x15\xA2$Ou\xCA\x13\xD8HZ\x8B\x84\xD00\xC2\xF2dsolcC\0\x08\x17\x003";
    /// The bytecode of the contract.
    pub static BLOCKCHAINEMAIL_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\x93W`\x005`\xE0\x1C\x80cxp\xFBJ\x11a\0fW\x80cxp\xFBJ\x14a\x013W\x80c\x81\x0B\x91x\x14a\x01FW\x80c\x85|\xDB\xB8\x14a\x01YW\x80c\xAA>\xF8\x0E\x14a\x01\x9DW\x80c\xBB]\xDB\x0F\x14a\x01\xBDW`\0\x80\xFD[\x80c\x11\xD5\x1E1\x14a\0\x98W\x80c\x15\xCB?\xFB\x14a\0\xC2W\x80c8\x82\xB5]\x14a\0\xFEW\x80c@.\x0FS\x14a\x01\x13W[`\0\x80\xFD[a\0\xABa\0\xA66`\x04a\nDV[a\x01\xD0V[`@Qa\0\xB9\x92\x91\x90a\n\xADV[`@Q\x80\x91\x03\x90\xF3[a\0\xE9a\0\xD06`\x04a\n\xF7V[`\0` \x81\x90R\x90\x81R`@\x90 \x80T`\x01\x90\x91\x01T\x82V[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01a\0\xB9V[a\x01\x11a\x01\x0C6`\x04a\x0B[V[a\x02\xFCV[\0[a\x01&a\x01!6`\x04a\x0B\xDCV[a\x04\"V[`@Qa\0\xB9\x91\x90a\x0C\x18V[a\x01&a\x01A6`\x04a\n\xF7V[a\x04\xE8V[a\x01\x11a\x01T6`\x04a\x0C+V[a\x06\x0EV[a\0\xE9a\x01g6`\x04a\n\xF7V[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R` \x81\x81R`@\x91\x82\x90 \x82Q\x80\x84\x01\x90\x93R\x80T\x80\x84R`\x01\x90\x91\x01T\x92\x90\x91\x01\x82\x90R\x91V[a\x01\xB0a\x01\xAB6`\x04a\x0CMV[a\x06mV[`@Qa\0\xB9\x91\x90a\x0C\x80V[a\x01\x11a\x01\xCB6`\x04a\x0C\xFAV[a\x07iV[`\x02` R`\0\x90\x81R`@\x90 \x80T\x81\x90a\x01\xEB\x90a\r\xBCV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x02\x17\x90a\r\xBCV[\x80\x15a\x02dW\x80`\x1F\x10a\x029Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x02dV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x02GW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90\x80`\x01\x01\x80Ta\x02y\x90a\r\xBCV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x02\xA5\x90a\r\xBCV[\x80\x15a\x02\xF2W\x80`\x1F\x10a\x02\xC7Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x02\xF2V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x02\xD5W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x82V[`\0\x80`\x01`\x01`\xA0\x1B\x03\x87\x163\x10a\x03\x16W\x863a\x03\x19V[3\x87[\x91P\x91P`\0\x82\x82`@Q` \x01a\x032\x92\x91\x90a\r\xF6V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P`\0a\x03U\x82a\x08\x87V[\x90P\x80\x15a\x03\x93W`@Qc\xC3p\x1A\xC9`\xE0\x1B\x81R`\x04\x81\x01\x83\x90R3`$\x82\x01R`\x01`\x01`\xA0\x1B\x03\x8A\x16`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x84\x163\x03a\x03\xDFW`\0\x82\x81R`\x02` R`@\x90 a\x03\xBC\x88\x8A\x83a\x0EnV[P`\0\x82\x81R`\x02` R`@\x90 `\x01\x01a\x03\xD9\x86\x88\x83a\x0EnV[Pa\x04\x17V[`\0\x82\x81R`\x02` R`@\x90 a\x03\xF8\x86\x88\x83a\x0EnV[P`\0\x82\x81R`\x02` R`@\x90 `\x01\x01a\x04\x15\x88\x8A\x83a\x0EnV[P[PPPPPPPPPV[`\x01` R\x82`\0R`@`\0 ` R\x81`\0R`@`\0 \x81\x81T\x81\x10a\x04JW`\0\x80\xFD[\x90`\0R` `\0 \x01`\0\x92P\x92PPP\x80Ta\x04g\x90a\r\xBCV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x04\x93\x90a\r\xBCV[\x80\x15a\x04\xE0W\x80`\x1F\x10a\x04\xB5Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x04\xE0V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x04\xC3W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81V[```\0\x80`\x01`\x01`\xA0\x1B\x03\x84\x163\x10a\x05\x04W\x833a\x05\x07V[3\x84[\x91P\x91P`\0\x82\x82`@Q` \x01a\x05 \x92\x91\x90a\r\xF6V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x82`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x03a\x05\xF2W`\0\x81\x81R`\x02` R`@\x90 \x80Ta\x05j\x90a\r\xBCV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x05\x96\x90a\r\xBCV[\x80\x15a\x05\xE3W\x80`\x1F\x10a\x05\xB8Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x05\xE3V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x05\xC6W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x93PPPP\x91\x90PV[`\0\x81\x81R`\x02` R`@\x90 `\x01\x01\x80Ta\x05j\x90a\r\xBCV[`@\x80Q\x80\x82\x01\x82R\x83\x81R` \x80\x82\x01\x84\x81R3`\0\x81\x81R\x92\x83\x90R\x84\x83 \x93Q\x84U\x90Q`\x01\x90\x93\x01\x92\x90\x92U\x91Q\x83\x92\x85\x92\x91\x7F\xC2\xA3A\xA2\xBB\x1D\xCD`x@Y\xFB\xC0\xDE\xD5,O\x1F\x80Y\r\x86\xFA\xDE+v\x18w!\x88\xB7\xFA\x91\x90\xA4PPV[`\x01`\x01`\xA0\x1B\x03\x80\x83\x16`\0\x90\x81R`\x01` \x90\x81R`@\x80\x83 \x93\x85\x16\x83R\x92\x81R\x82\x82 \x80T\x84Q\x81\x84\x02\x81\x01\x84\x01\x90\x95R\x80\x85R``\x94\x93\x91\x92\x90\x91\x90\x84\x01[\x82\x82\x10\x15a\x07]W\x83\x82\x90`\0R` `\0 \x01\x80Ta\x06\xD0\x90a\r\xBCV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x06\xFC\x90a\r\xBCV[\x80\x15a\x07IW\x80`\x1F\x10a\x07\x1EWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x07IV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x07,W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\x06\xB1V[PPPP\x90P\x92\x91PPV[`\0a\x07u3\x84a\t\xE5V[PP\x90Pa\x07\x82\x81a\x08\x87V[a\x07\xB7W`@Qc/0\"\x17`\xE2\x1B\x81R`\x04\x81\x01\x82\x90R3`$\x82\x01R`\x01`\x01`\xA0\x1B\x03\x84\x16`D\x82\x01R`d\x01a\x03\x8AV[`@\x80QB` \x82\x01R`\0\x91\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x82\x81`@Q` \x01a\x07\xE9\x92\x91\x90a\x0F/V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x81R3`\0\x90\x81R`\x01` \x81\x81R\x83\x83 `\x01`\x01`\xA0\x1B\x03\x8A\x16\x84R\x81R\x92\x82 \x80T\x91\x82\x01\x81U\x82R\x91\x90 \x91\x94P\x01a\x084\x84\x82a\x0F^V[PB\x84`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x7F\xFA\x9D \x8F\x16\xE4\xEB_\xE2\xB7\xDA\xCC\x89\xDC\x18\xB1,\x13\xD9\x16^elU\x1C\x83D\x1E\xAD\xF5\xB1\xA2\x86`@Qa\x08y\x91\x90a\x0C\x18V[`@Q\x80\x91\x03\x90\xA4PPPPV[`\0\x81\x81R`\x02` R`@\x80\x82 \x81Q\x80\x83\x01\x90\x92R\x80T\x83\x92\x91\x90\x82\x90\x82\x90a\x08\xB1\x90a\r\xBCV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x08\xDD\x90a\r\xBCV[\x80\x15a\t*W\x80`\x1F\x10a\x08\xFFWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\t*V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\t\rW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x01\x82\x01\x80Ta\tC\x90a\r\xBCV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\to\x90a\r\xBCV[\x80\x15a\t\xBCW\x80`\x1F\x10a\t\x91Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\t\xBCV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\t\x9FW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPP\x91\x90\x92RPP\x81QQ\x91\x92PP\x15\x80\x15\x90a\t\xDEWP` \x81\x01QQ\x15\x15[\x93\x92PPPV[`\0\x80`\0\x83`\x01`\x01`\xA0\x1B\x03\x16\x85`\x01`\x01`\xA0\x1B\x03\x16\x10a\n\nW\x83\x85a\n\rV[\x84\x84[`@Q\x91\x93P\x91Pa\n%\x90\x83\x90\x83\x90` \x01a\r\xF6V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x92P\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15a\nVW`\0\x80\xFD[P5\x91\x90PV[`\0[\x83\x81\x10\x15a\nxW\x81\x81\x01Q\x83\x82\x01R` \x01a\n`V[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra\n\x99\x81` \x86\x01` \x86\x01a\n]V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[`@\x81R`\0a\n\xC0`@\x83\x01\x85a\n\x81V[\x82\x81\x03` \x84\x01Ra\n\xD2\x81\x85a\n\x81V[\x95\x94PPPPPV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\n\xF2W`\0\x80\xFD[\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x0B\tW`\0\x80\xFD[a\t\xDE\x82a\n\xDBV[`\0\x80\x83`\x1F\x84\x01\x12a\x0B$W`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0B<W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\x0BTW`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80`\0\x80`\0``\x86\x88\x03\x12\x15a\x0BsW`\0\x80\xFD[a\x0B|\x86a\n\xDBV[\x94P` \x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x0B\x99W`\0\x80\xFD[a\x0B\xA5\x89\x83\x8A\x01a\x0B\x12V[\x90\x96P\x94P`@\x88\x015\x91P\x80\x82\x11\x15a\x0B\xBEW`\0\x80\xFD[Pa\x0B\xCB\x88\x82\x89\x01a\x0B\x12V[\x96\x99\x95\x98P\x93\x96P\x92\x94\x93\x92PPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x0B\xF1W`\0\x80\xFD[a\x0B\xFA\x84a\n\xDBV[\x92Pa\x0C\x08` \x85\x01a\n\xDBV[\x91P`@\x84\x015\x90P\x92P\x92P\x92V[` \x81R`\0a\t\xDE` \x83\x01\x84a\n\x81V[`\0\x80`@\x83\x85\x03\x12\x15a\x0C>W`\0\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[`\0\x80`@\x83\x85\x03\x12\x15a\x0C`W`\0\x80\xFD[a\x0Ci\x83a\n\xDBV[\x91Pa\x0Cw` \x84\x01a\n\xDBV[\x90P\x92P\x92\x90PV[`\0` \x80\x83\x01` \x84R\x80\x85Q\x80\x83R`@\x86\x01\x91P`@\x81`\x05\x1B\x87\x01\x01\x92P` \x87\x01`\0[\x82\x81\x10\x15a\x0C\xD7W`?\x19\x88\x86\x03\x01\x84Ra\x0C\xC5\x85\x83Qa\n\x81V[\x94P\x92\x85\x01\x92\x90\x85\x01\x90`\x01\x01a\x0C\xA9V[P\x92\x97\x96PPPPPPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0\x80`@\x83\x85\x03\x12\x15a\r\rW`\0\x80\xFD[a\r\x16\x83a\n\xDBV[\x91P` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\r3W`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\rGW`\0\x80\xFD[\x815\x81\x81\x11\x15a\rYWa\rYa\x0C\xE4V[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x83\x82\x11\x81\x83\x10\x17\x15a\r\x81Wa\r\x81a\x0C\xE4V[\x81`@R\x82\x81R\x88` \x84\x87\x01\x01\x11\x15a\r\x9AW`\0\x80\xFD[\x82` \x86\x01` \x83\x017`\0` \x84\x83\x01\x01R\x80\x95PPPPPP\x92P\x92\x90PV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\r\xD0W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\r\xF0WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19``\x93\x84\x1B\x81\x16\x82R\x91\x90\x92\x1B\x16`\x14\x82\x01R`(\x01\x90V[`\x1F\x82\x11\x15a\x0EiW`\0\x81`\0R` `\0 `\x1F\x85\x01`\x05\x1C\x81\x01` \x86\x10\x15a\x0EFWP\x80[`\x1F\x85\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\x0EeW\x82\x81U`\x01\x01a\x0ERV[PPP[PPPV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11\x15a\x0E\x86Wa\x0E\x86a\x0C\xE4V[a\x0E\x9A\x83a\x0E\x94\x83Ta\r\xBCV[\x83a\x0E\x1DV[`\0`\x1F\x84\x11`\x01\x81\x14a\x0E\xCEW`\0\x85\x15a\x0E\xB6WP\x83\x82\x015[`\0\x19`\x03\x87\x90\x1B\x1C\x19\x16`\x01\x86\x90\x1B\x17\x83Ua\x0F(V[`\0\x83\x81R` \x90 `\x1F\x19\x86\x16\x90\x83[\x82\x81\x10\x15a\x0E\xFFW\x86\x85\x015\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01a\x0E\xDFV[P\x86\x82\x10\x15a\x0F\x1CW`\0\x19`\xF8\x88`\x03\x1B\x16\x1C\x19\x84\x87\x015\x16\x81U[PP`\x01\x85`\x01\x1B\x01\x83U[PPPPPV[`\0\x83Qa\x0FA\x81\x84` \x88\x01a\n]V[\x83Q\x90\x83\x01\x90a\x0FU\x81\x83` \x88\x01a\n]V[\x01\x94\x93PPPPV[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0FxWa\x0Fxa\x0C\xE4V[a\x0F\x8C\x81a\x0F\x86\x84Ta\r\xBCV[\x84a\x0E\x1DV[` \x80`\x1F\x83\x11`\x01\x81\x14a\x0F\xC1W`\0\x84\x15a\x0F\xA9WP\x85\x83\x01Q[`\0\x19`\x03\x86\x90\x1B\x1C\x19\x16`\x01\x85\x90\x1B\x17\x85Ua\x0EeV[`\0\x85\x81R` \x81 `\x1F\x19\x86\x16\x91[\x82\x81\x10\x15a\x0F\xF0W\x88\x86\x01Q\x82U\x94\x84\x01\x94`\x01\x90\x91\x01\x90\x84\x01a\x0F\xD1V[P\x85\x82\x10\x15a\x10\x0EW\x87\x85\x01Q`\0\x19`\x03\x88\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPPP`\x01\x90\x81\x1B\x01\x90UPV\xFE\xA2dipfsX\"\x12 O\0\xAC\xEC>\x10\xB0\xAD\x10\xCFZ\xC1\xCCz,\x91_\x0E\xE3\\\xEA\x9A#\x83\xC6\xC7\x9D\x01 \xA5\xC8\xD2dsolcC\0\x08\x17\x003";
    /// The deployed bytecode of the contract.
    pub static BLOCKCHAINEMAIL_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct BlockchainEmail<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for BlockchainEmail<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for BlockchainEmail<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for BlockchainEmail<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for BlockchainEmail<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(BlockchainEmail))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> BlockchainEmail<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    BLOCKCHAINEMAIL_ABI.clone(),
                    client,
                ),
            )
        }
        /// Constructs the general purpose `Deployer` instance based on the provided constructor arguments and sends it.
        /// Returns a new instance of a deployer that returns an instance of this contract after sending the transaction
        ///
        /// Notes:
        /// - If there are no constructor arguments, you should pass `()` as the argument.
        /// - The default poll duration is 7 seconds.
        /// - The default number of confirmations is 1 block.
        ///
        ///
        /// # Example
        ///
        /// Generate contract bindings with `abigen!` and deploy a new contract instance.
        ///
        /// *Note*: this requires a `bytecode` and `abi` object in the `greeter.json` artifact.
        ///
        /// ```ignore
        /// # async fn deploy<M: ethers::providers::Middleware>(client: ::std::sync::Arc<M>) {
        ///     abigen!(Greeter, "../greeter.json");
        ///
        ///    let greeter_contract = Greeter::deploy(client, "Hello world!".to_string()).unwrap().send().await.unwrap();
        ///    let msg = greeter_contract.greet().call().await.unwrap();
        /// # }
        /// ```
        pub fn deploy<T: ::ethers::core::abi::Tokenize>(
            client: ::std::sync::Arc<M>,
            constructor_args: T,
        ) -> ::core::result::Result<
            ::ethers::contract::builders::ContractDeployer<M, Self>,
            ::ethers::contract::ContractError<M>,
        > {
            let factory = ::ethers::contract::ContractFactory::new(
                BLOCKCHAINEMAIL_ABI.clone(),
                BLOCKCHAINEMAIL_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `addEncryptedSecretKeys` (0x3882b55d) function
        pub fn add_encrypted_secret_keys(
            &self,
            recipient: ::ethers::core::types::Address,
            sender_encrypted_secret_key: ::ethers::core::types::Bytes,
            recipient_encrypted_secret_key: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [56, 130, 181, 93],
                    (
                        recipient,
                        sender_encrypted_secret_key,
                        recipient_encrypted_secret_key,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `addPublicKey` (0x810b9178) function
        pub fn add_public_key(
            &self,
            public_key_half_1: [u8; 32],
            public_key_half_2: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([129, 11, 145, 120], (public_key_half_1, public_key_half_2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getEncryptedSecretKey` (0x7870fb4a) function
        pub fn get_encrypted_secret_key(
            &self,
            recipient: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Bytes,
        > {
            self.0
                .method_hash([120, 112, 251, 74], recipient)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getMessages` (0xaa3ef80e) function
        pub fn get_messages(
            &self,
            sender: ::ethers::core::types::Address,
            recipient: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::Bytes>,
        > {
            self.0
                .method_hash([170, 62, 248, 14], (sender, recipient))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getPublicKey` (0x857cdbb8) function
        pub fn get_public_key(
            &self,
            user_address: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ([u8; 32], [u8; 32])> {
            self.0
                .method_hash([133, 124, 219, 184], user_address)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `s_addressToPublicKey` (0x15cb3ffb) function
        pub fn s_address_to_public_key(
            &self,
            user: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ([u8; 32], [u8; 32])> {
            self.0
                .method_hash([21, 203, 63, 251], user)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `s_participantsToEncryptedSk` (0x11d51e31) function
        pub fn s_participants_to_encrypted_sk(
            &self,
            participants: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::Bytes, ::ethers::core::types::Bytes),
        > {
            self.0
                .method_hash([17, 213, 30, 49], participants)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `s_senderToRecipientMessages` (0x402e0f53) function
        pub fn s_sender_to_recipient_messages(
            &self,
            sender: ::ethers::core::types::Address,
            recipient: ::ethers::core::types::Address,
            p2: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Bytes,
        > {
            self.0
                .method_hash([64, 46, 15, 83], (sender, recipient, p2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `sendMessage` (0xbb5ddb0f) function
        pub fn send_message(
            &self,
            recipient: ::ethers::core::types::Address,
            message: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([187, 93, 219, 15], (recipient, message))
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `MessageSent` event
        pub fn message_sent_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            MessageSentFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `PublicKeyAdded` event
        pub fn public_key_added_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            PublicKeyAddedFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            BlockchainEmailEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for BlockchainEmail<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `Messanger__SecretKeyAlreadyInitialized` with signature `Messanger__SecretKeyAlreadyInitialized(bytes32,address,address)` and selector `0xc3701ac9`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(
        name = "Messanger__SecretKeyAlreadyInitialized",
        abi = "Messanger__SecretKeyAlreadyInitialized(bytes32,address,address)"
    )]
    pub struct Messanger__SecretKeyAlreadyInitialized {
        pub participant: [u8; 32],
        pub sender: ::ethers::core::types::Address,
        pub recipient: ::ethers::core::types::Address,
    }
    ///Custom Error type `Messanger__SecretKeyNotInitialized` with signature `Messanger__SecretKeyNotInitialized(bytes32,address,address)` and selector `0xbcc0885c`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(
        name = "Messanger__SecretKeyNotInitialized",
        abi = "Messanger__SecretKeyNotInitialized(bytes32,address,address)"
    )]
    pub struct Messanger__SecretKeyNotInitialized {
        pub participant: [u8; 32],
        pub sender: ::ethers::core::types::Address,
        pub recipient: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's custom errors
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum BlockchainEmailErrors {
        Messanger__SecretKeyAlreadyInitialized(Messanger__SecretKeyAlreadyInitialized),
        Messanger__SecretKeyNotInitialized(Messanger__SecretKeyNotInitialized),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for BlockchainEmailErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <Messanger__SecretKeyAlreadyInitialized as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Messanger__SecretKeyAlreadyInitialized(decoded));
            }
            if let Ok(decoded) = <Messanger__SecretKeyNotInitialized as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Messanger__SecretKeyNotInitialized(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for BlockchainEmailErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::Messanger__SecretKeyAlreadyInitialized(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Messanger__SecretKeyNotInitialized(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for BlockchainEmailErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <Messanger__SecretKeyAlreadyInitialized as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <Messanger__SecretKeyNotInitialized as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for BlockchainEmailErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Messanger__SecretKeyAlreadyInitialized(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Messanger__SecretKeyNotInitialized(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for BlockchainEmailErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<Messanger__SecretKeyAlreadyInitialized>
    for BlockchainEmailErrors {
        fn from(value: Messanger__SecretKeyAlreadyInitialized) -> Self {
            Self::Messanger__SecretKeyAlreadyInitialized(value)
        }
    }
    impl ::core::convert::From<Messanger__SecretKeyNotInitialized>
    for BlockchainEmailErrors {
        fn from(value: Messanger__SecretKeyNotInitialized) -> Self {
            Self::Messanger__SecretKeyNotInitialized(value)
        }
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "MessageSent", abi = "MessageSent(address,address,bytes,uint256)")]
    pub struct MessageSentFilter {
        #[ethevent(indexed)]
        pub sender: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub recipient: ::ethers::core::types::Address,
        pub content: ::ethers::core::types::Bytes,
        #[ethevent(indexed)]
        pub timestamp: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "PublicKeyAdded", abi = "PublicKeyAdded(address,bytes32,bytes32)")]
    pub struct PublicKeyAddedFilter {
        #[ethevent(indexed)]
        pub user: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub public_key_half_1: [u8; 32],
        #[ethevent(indexed)]
        pub public_key_half_2: [u8; 32],
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum BlockchainEmailEvents {
        MessageSentFilter(MessageSentFilter),
        PublicKeyAddedFilter(PublicKeyAddedFilter),
    }
    impl ::ethers::contract::EthLogDecode for BlockchainEmailEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = MessageSentFilter::decode_log(log) {
                return Ok(BlockchainEmailEvents::MessageSentFilter(decoded));
            }
            if let Ok(decoded) = PublicKeyAddedFilter::decode_log(log) {
                return Ok(BlockchainEmailEvents::PublicKeyAddedFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for BlockchainEmailEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::MessageSentFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::PublicKeyAddedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<MessageSentFilter> for BlockchainEmailEvents {
        fn from(value: MessageSentFilter) -> Self {
            Self::MessageSentFilter(value)
        }
    }
    impl ::core::convert::From<PublicKeyAddedFilter> for BlockchainEmailEvents {
        fn from(value: PublicKeyAddedFilter) -> Self {
            Self::PublicKeyAddedFilter(value)
        }
    }
    ///Container type for all input parameters for the `addEncryptedSecretKeys` function with signature `addEncryptedSecretKeys(address,bytes,bytes)` and selector `0x3882b55d`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "addEncryptedSecretKeys",
        abi = "addEncryptedSecretKeys(address,bytes,bytes)"
    )]
    pub struct AddEncryptedSecretKeysCall {
        pub recipient: ::ethers::core::types::Address,
        pub sender_encrypted_secret_key: ::ethers::core::types::Bytes,
        pub recipient_encrypted_secret_key: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `addPublicKey` function with signature `addPublicKey(bytes32,bytes32)` and selector `0x810b9178`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "addPublicKey", abi = "addPublicKey(bytes32,bytes32)")]
    pub struct AddPublicKeyCall {
        pub public_key_half_1: [u8; 32],
        pub public_key_half_2: [u8; 32],
    }
    ///Container type for all input parameters for the `getEncryptedSecretKey` function with signature `getEncryptedSecretKey(address)` and selector `0x7870fb4a`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "getEncryptedSecretKey", abi = "getEncryptedSecretKey(address)")]
    pub struct GetEncryptedSecretKeyCall {
        pub recipient: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getMessages` function with signature `getMessages(address,address)` and selector `0xaa3ef80e`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "getMessages", abi = "getMessages(address,address)")]
    pub struct GetMessagesCall {
        pub sender: ::ethers::core::types::Address,
        pub recipient: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getPublicKey` function with signature `getPublicKey(address)` and selector `0x857cdbb8`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "getPublicKey", abi = "getPublicKey(address)")]
    pub struct GetPublicKeyCall {
        pub user_address: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `s_addressToPublicKey` function with signature `s_addressToPublicKey(address)` and selector `0x15cb3ffb`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "s_addressToPublicKey", abi = "s_addressToPublicKey(address)")]
    pub struct SAddressToPublicKeyCall {
        pub user: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `s_participantsToEncryptedSk` function with signature `s_participantsToEncryptedSk(bytes32)` and selector `0x11d51e31`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "s_participantsToEncryptedSk",
        abi = "s_participantsToEncryptedSk(bytes32)"
    )]
    pub struct SParticipantsToEncryptedSkCall {
        pub participants: [u8; 32],
    }
    ///Container type for all input parameters for the `s_senderToRecipientMessages` function with signature `s_senderToRecipientMessages(address,address,uint256)` and selector `0x402e0f53`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "s_senderToRecipientMessages",
        abi = "s_senderToRecipientMessages(address,address,uint256)"
    )]
    pub struct SSenderToRecipientMessagesCall {
        pub sender: ::ethers::core::types::Address,
        pub recipient: ::ethers::core::types::Address,
        pub p2: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `sendMessage` function with signature `sendMessage(address,bytes)` and selector `0xbb5ddb0f`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "sendMessage", abi = "sendMessage(address,bytes)")]
    pub struct SendMessageCall {
        pub recipient: ::ethers::core::types::Address,
        pub message: ::ethers::core::types::Bytes,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum BlockchainEmailCalls {
        AddEncryptedSecretKeys(AddEncryptedSecretKeysCall),
        AddPublicKey(AddPublicKeyCall),
        GetEncryptedSecretKey(GetEncryptedSecretKeyCall),
        GetMessages(GetMessagesCall),
        GetPublicKey(GetPublicKeyCall),
        SAddressToPublicKey(SAddressToPublicKeyCall),
        SParticipantsToEncryptedSk(SParticipantsToEncryptedSkCall),
        SSenderToRecipientMessages(SSenderToRecipientMessagesCall),
        SendMessage(SendMessageCall),
    }
    impl ::ethers::core::abi::AbiDecode for BlockchainEmailCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <AddEncryptedSecretKeysCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AddEncryptedSecretKeys(decoded));
            }
            if let Ok(decoded) = <AddPublicKeyCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AddPublicKey(decoded));
            }
            if let Ok(decoded) = <GetEncryptedSecretKeyCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetEncryptedSecretKey(decoded));
            }
            if let Ok(decoded) = <GetMessagesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetMessages(decoded));
            }
            if let Ok(decoded) = <GetPublicKeyCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetPublicKey(decoded));
            }
            if let Ok(decoded) = <SAddressToPublicKeyCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SAddressToPublicKey(decoded));
            }
            if let Ok(decoded) = <SParticipantsToEncryptedSkCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SParticipantsToEncryptedSk(decoded));
            }
            if let Ok(decoded) = <SSenderToRecipientMessagesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SSenderToRecipientMessages(decoded));
            }
            if let Ok(decoded) = <SendMessageCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SendMessage(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for BlockchainEmailCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AddEncryptedSecretKeys(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AddPublicKey(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetEncryptedSecretKey(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetMessages(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetPublicKey(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SAddressToPublicKey(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SParticipantsToEncryptedSk(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SSenderToRecipientMessages(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SendMessage(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for BlockchainEmailCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AddEncryptedSecretKeys(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AddPublicKey(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetEncryptedSecretKey(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetMessages(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetPublicKey(element) => ::core::fmt::Display::fmt(element, f),
                Self::SAddressToPublicKey(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SParticipantsToEncryptedSk(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SSenderToRecipientMessages(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SendMessage(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AddEncryptedSecretKeysCall> for BlockchainEmailCalls {
        fn from(value: AddEncryptedSecretKeysCall) -> Self {
            Self::AddEncryptedSecretKeys(value)
        }
    }
    impl ::core::convert::From<AddPublicKeyCall> for BlockchainEmailCalls {
        fn from(value: AddPublicKeyCall) -> Self {
            Self::AddPublicKey(value)
        }
    }
    impl ::core::convert::From<GetEncryptedSecretKeyCall> for BlockchainEmailCalls {
        fn from(value: GetEncryptedSecretKeyCall) -> Self {
            Self::GetEncryptedSecretKey(value)
        }
    }
    impl ::core::convert::From<GetMessagesCall> for BlockchainEmailCalls {
        fn from(value: GetMessagesCall) -> Self {
            Self::GetMessages(value)
        }
    }
    impl ::core::convert::From<GetPublicKeyCall> for BlockchainEmailCalls {
        fn from(value: GetPublicKeyCall) -> Self {
            Self::GetPublicKey(value)
        }
    }
    impl ::core::convert::From<SAddressToPublicKeyCall> for BlockchainEmailCalls {
        fn from(value: SAddressToPublicKeyCall) -> Self {
            Self::SAddressToPublicKey(value)
        }
    }
    impl ::core::convert::From<SParticipantsToEncryptedSkCall> for BlockchainEmailCalls {
        fn from(value: SParticipantsToEncryptedSkCall) -> Self {
            Self::SParticipantsToEncryptedSk(value)
        }
    }
    impl ::core::convert::From<SSenderToRecipientMessagesCall> for BlockchainEmailCalls {
        fn from(value: SSenderToRecipientMessagesCall) -> Self {
            Self::SSenderToRecipientMessages(value)
        }
    }
    impl ::core::convert::From<SendMessageCall> for BlockchainEmailCalls {
        fn from(value: SendMessageCall) -> Self {
            Self::SendMessage(value)
        }
    }
    ///Container type for all return fields from the `getEncryptedSecretKey` function with signature `getEncryptedSecretKey(address)` and selector `0x7870fb4a`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct GetEncryptedSecretKeyReturn(pub ::ethers::core::types::Bytes);
    ///Container type for all return fields from the `getMessages` function with signature `getMessages(address,address)` and selector `0xaa3ef80e`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct GetMessagesReturn(pub ::std::vec::Vec<::ethers::core::types::Bytes>);
    ///Container type for all return fields from the `getPublicKey` function with signature `getPublicKey(address)` and selector `0x857cdbb8`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct GetPublicKeyReturn(pub [u8; 32], pub [u8; 32]);
    ///Container type for all return fields from the `s_addressToPublicKey` function with signature `s_addressToPublicKey(address)` and selector `0x15cb3ffb`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct SAddressToPublicKeyReturn {
        pub half_1: [u8; 32],
        pub half_2: [u8; 32],
    }
    ///Container type for all return fields from the `s_participantsToEncryptedSk` function with signature `s_participantsToEncryptedSk(bytes32)` and selector `0x11d51e31`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct SParticipantsToEncryptedSkReturn {
        pub lower_address_sk: ::ethers::core::types::Bytes,
        pub higher_address_sk: ::ethers::core::types::Bytes,
    }
    ///Container type for all return fields from the `s_senderToRecipientMessages` function with signature `s_senderToRecipientMessages(address,address,uint256)` and selector `0x402e0f53`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct SSenderToRecipientMessagesReturn {
        pub messages: ::ethers::core::types::Bytes,
    }
}
