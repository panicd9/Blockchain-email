#![warn(unused_extern_crates)]
mod cli;
mod commands;
use clap::Parser;
use cli::MessangerArgs;
use commands::*;

#[tokio::main]
async fn main() {
    let args = MessangerArgs::parse();

    match args.command {
        cli::Command::SendMessage(send_message_args) => {
            handle_send_message_command(send_message_args).await
        }
        cli::Command::InitChat(init_chat_args) => handle_init_chat_command(init_chat_args).await,
        cli::Command::AddPublicKey(add_public_key_args) => {
            handle_add_public_key_command(add_public_key_args).await
        }
        cli::Command::GetMessages(get_messages_args) => {
            handle_get_messages_command(get_messages_args).await
        }
        cli::Command::Test {  } => handle_test_command().await,
    }
}

// https://ethereum.stackexchange.com/a/896
// 30 gwei gas
// 1900 $USD per ETH

// ATM 1.38 $USD to send "Hey boss!"
// I calculated this without timestamp added to the message
// 37,324,800 $USD per GB of data
