use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct MessangerArgs {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Send a message
    SendMessage(SendMessageArgs),
    /// Initialize a chat - generate secret key, encrypt it with sender's and recipient's public keys and send it to the contract
    InitChat(InitChatArgs),
    /// Add public key to the contract - used to initialize chat
    AddPublicKey(AddPublicKeyArgs),
    /// Get messages
    GetMessages(GetMessagesArgs),
    /// Test simulation
    Test,
}

#[derive(Debug, Args)]
pub struct SendMessageArgs {
    #[arg(short, long, default_value_t =  String::from("0x5FbDB2315678afecb367f032d93F642f64180aa3"))]
    pub contract_address: String,
    #[arg(short, long, default_value_t =  String::from("0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80"))]
    pub private_key: String,
    #[arg(short, long)]
    pub recipient: String,
    #[arg(short, long)]
    pub message: String,
    #[arg(short = 'u', long, default_value_t =  String::from("http://127.0.0.1:8545"))]
    pub rpc_url: String,
    #[arg(short, long)]
    pub verbose: bool,
}

#[derive(Debug, Args)]
pub struct InitChatArgs {
    #[arg(short, long, default_value_t =  String::from("0x5FbDB2315678afecb367f032d93F642f64180aa3"))]
    pub contract_address: String,
    #[arg(short, long, default_value_t =  String::from("0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80"))]
    pub private_key: String,
    #[arg(short, long)]
    pub recipient: String,
    #[arg(short = 'u', long, default_value_t =  String::from("http://127.0.0.1:8545"))]
    pub rpc_url: String,
    #[arg(short, long)]
    pub verbose: bool,
}

#[derive(Debug, Args)]
pub struct AddPublicKeyArgs {
    #[arg(short, long, default_value_t =  String::from("0x5FbDB2315678afecb367f032d93F642f64180aa3"))]
    pub contract_address: String,
    #[arg(short, long, default_value_t =  String::from("0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80"))]
    pub private_key: String,
    #[arg(short = 'u', long, default_value_t =  String::from("http://127.0.0.1:8545"))]
    pub rpc_url: String,
    #[arg(short, long)]
    pub verbose: bool,
}

#[derive(Debug, Args)]
pub struct GetMessagesArgs {
    #[arg(short, long, default_value_t =  String::from("0x5FbDB2315678afecb367f032d93F642f64180aa3"))]
    pub contract_address: String,
    #[arg(short, long, default_value_t =  String::from("0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80"))]
    pub private_key: String,
    #[arg(short = 'u', long, default_value_t =  String::from("http://127.0.0.1:8545"))]
    pub rpc_url: String,
    #[arg(short, long)]
    pub recipient: String,
    #[arg(short, long)]
    pub verbose: bool,
}

pub trait NormalizeArgs {
    fn validate_args(&mut self) -> Result<(), String>;
}

impl NormalizeArgs for SendMessageArgs {
    fn validate_args(&mut self) -> Result<(), String> {
        if self.private_key.starts_with("0x") {
            self.private_key = self.private_key[2..].to_string();
        }

        if self.private_key.len() != 64 {
            return Err(
                "Private key must be 64 hex characters long (66 with 0x prefix)".to_string(),
            );
        }

        if self.contract_address.len() != 42 {
            return Err(
                "Contract address must be 40 hex characters long (42 with 0x prefix)".to_string(),
            );
        }

        if self.recipient.len() != 42 {
            return Err("Recipient must be 40 hex characters long (42 with 0x prefix)".to_string());
        }

        if self.message.len() == 0 {
            return Err("Message must not be empty".to_string());
        }

        if self.rpc_url.len() == 0 {
            return Err("RPC URL must not be empty".to_string());
        }

        Ok(())
    }
}

impl NormalizeArgs for InitChatArgs {
    fn validate_args(&mut self) -> Result<(), String> {
        if self.private_key.starts_with("0x") {
            self.private_key = self.private_key[2..].to_string();
        }

        if self.private_key.len() != 64 {
            return Err(
                "Private key must be 64 hex characters long (66 with 0x prefix)".to_string(),
            );
        }

        if self.contract_address.len() != 42 {
            return Err(
                "Contract address must be 40 hex characters long (42 with 0x prefix)".to_string(),
            );
        }

        if self.recipient.len() != 42 {
            return Err("Recipient must be 40 hex characters long (42 with 0x prefix)".to_string());
        }

        if self.rpc_url.len() == 0 {
            return Err("RPC URL must not be empty".to_string());
        }

        Ok(())
    }
}

impl NormalizeArgs for AddPublicKeyArgs {
    fn validate_args(&mut self) -> Result<(), String> {
        if self.private_key.starts_with("0x") {
            self.private_key = self.private_key[2..].to_string();
        }

        if self.private_key.len() != 64 {
            return Err(
                "Private key must be 64 hex characters long (66 with 0x prefix)".to_string(),
            );
        }

        if self.contract_address.len() != 42 {
            return Err(
                "Contract address must be 40 hex characters long (42 with 0x prefix)".to_string(),
            );
        }

        if self.rpc_url.len() == 0 {
            return Err("RPC URL must not be empty".to_string());
        }

        Ok(())
    }
}

impl NormalizeArgs for GetMessagesArgs {
    fn validate_args(&mut self) -> Result<(), String> {
        if self.private_key.starts_with("0x") {
            self.private_key = self.private_key[2..].to_string();
        }

        if self.private_key.len() != 64 {
            return Err(
                "Private key must be 64 hex characters long (66 with 0x prefix)".to_string(),
            );
        }

        if self.contract_address.len() != 42 {
            return Err(
                "Contract address must be 40 hex characters long (42 with 0x prefix)".to_string(),
            );
        }

        if self.recipient.len() != 42 {
            return Err("Recipient must be 40 hex characters long (42 with 0x prefix)".to_string());
        }

        if self.rpc_url.len() == 0 {
            return Err("RPC URL must not be empty".to_string());
        }

        Ok(())
    }
}
