use async_std::io::BufReader;
use async_std::prelude::*;
use async_std::{task, io, net};
use std::sync::Arc;

use chat::utils::{self, ChatResult};
use chat::{Client, Server};

async fn send(mut send: net::TcpStream) -> ChatResult<()>{
    println!("Options: \n join CHAT \n post CHAT MESSAGE \n");

    let mut options = io::BufReader::new(io::stdin()).lines();

    while_let Some(option_result) = options.next().await {
        let opt = option_result?;
        let req = match parse_int(&opt) {
            Some(0) => break,
            Some(1) => {
                let chat_name = get_chat_name(&opt);
                Client::Join { chat_name }
            }
            Some(2) => {
                let (chat_name, message) = get_chat_name_and_message(&opt);
                Client::Post { chat_name, message }
            }
            _ => {
                println!("Invalid option");
                continue;
            }
        };
        utils::send_json(&mut send, &req).await?;
        send.flush().await?;
    }
}
fn main() {
    println!("Hello, world!");
}
