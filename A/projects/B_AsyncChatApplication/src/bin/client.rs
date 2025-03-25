use async_std::io::BufReader;
use async_std::prelude::*;
use async_std::{task, io, net};
use std::sync::Arc;

use chat::utils::{self, ChatResult};
use chat::{Client, Server};

async fn send_generated_by_Copilot(mut send: net::TcpStream) -> ChatResult<()> {
    let mut stdin = BufReader::new(io::stdin());
    let mut line = String::new();

    while stdin.read_line(&mut line).await? > 0 {
        let client = match line.trim() {
            "/join" => {
                println!("Enter chat name:");
                line.clear();
                stdin.read_line(&mut line).await?;
                Client::Join {
                    chat_name: Arc::new(line.trim().to_string()),
                }
            }
            _ => {
                Client::Post {
                    chat_name: Arc::new("main".to_string()),
                    message: Arc::new(line.trim().to_string()),
                }
            }
        };

        utils::send_json(&mut send, &client).await?;
        line.clear();
    }

    Ok(())
}

fn get_value(mut input: &str) -> Option<(&str, &str)>{
    input = input.trim_start();

    if input.is_empty() {
        return None;
    }   

    match input.find(chat::is_whitespace){
        Some(whitespace) => Some((&input[0..whitespace], &input[whitespace..])),
        None => Some((input, "")),
    }
}
fn parse_input(line: &str) -> Option<Client>{
    let (input, remainder) = get_value(line)?;

    if input == "join" {
        let ( chat_name,remainder) = get_value(remainder)?;

        if !remainder.trim_start().is_empty(){
            eprintln!("Invalid input");
            return None;
        }   

        return Some(Client::Join {
            chat_name: Arc::new(chat_name.to_string()),
        })
    } else if input == "post" {
        let (chat, remainder) = get_value(remainder)?;
        
        let message = remainder.trim_start().to_string();


        return Some(Client::Post {
            chat_name: Arc::new(input.to_string()),
            message: Arc::new(message.to_string()),
        })
    } else {
        eprintln!("Invalid input");
        None
    }
}
async fn send(mut send: net::TcpStream) -> ChatResult<()> {
    println!("Options: \njoin CHAT\npost CHAT MESSAGE");
    let mut options = io::BufReader::new(io::stdin()).lines();

    while let Some(option_result) = options.next().await {
        let opt = option_result?;
        let req = match parse_input(&opt) {
            Ok(req) => req,
            Err(e) => {
                eprintln!("{}", e);
                continue;
            }, 
            None => continue,
        };
        utils::send_json(&mut send, &req).await?;
        send.flush().await?;
    }
    Ok(())
}

async fn messages(server: net::TcpStream) -> ChatResult<()> {
    let mut incoming = BufReader::new(server).lines();

    let buf = io::BufReader::new(server);
    let mut stream = utils::receive(buf);

    while let Some(msg) = stream.next().await {
        let msg = msg?;
        match msg {
            Server::Message { chat_name, message } => {
                println!("chat Name: {} \n, Message: {}\n", chat_name, message);
            }
            Server::Error(e) => {
                eprintln!("Error received: {}", e);
            }
        }
    }

    Ok(())
}