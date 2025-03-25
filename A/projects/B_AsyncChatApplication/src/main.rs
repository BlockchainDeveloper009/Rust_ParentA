
use async_std::prelude::*;
use async_std::{task, io, net};
use std::sync::Arc;

use chat::utils::{self, ChatResult};
use chats::{Client, Server};


mod connection;
mod chats;
mod chats_map;
use connection::handle;

fn log_error(result: ChatResult<()>){
    if let Err(error) =  result {
        println!("Error: {}", error );
    }
}

// async fn send(mut send: net::TcpStream) -> ChatResult<()>{
//     println!("Options: \n join CHAT \n post CHAT MESSAGE \n");

//     let mut options = io::BufReader::new(io::stdin()).lines();

//     while_let Some(option_result) = options.next().await {
//         let opt = option_result?;
//         let req = match parse_int(&opt) {
//             Some(0) => break,
//             Some(1) => {
//                 let chat_name = get_chat_name(&opt);
//                 Client::Join { chat_name }
//             }
//             Some(2) => {
//                 let (chat_name, message) = get_chat_name_and_message(&opt);
//                 Client::Post { chat_name, message }
//             }
//             _ => {
//                 println!("Invalid option");
//                 continue;
//             }
//         };
//         utils::send_json(&mut send, &req).await?;
//         send.flush().await?;
//     }
//     Ok(())
// }
fn main() -> ChatResult<()> {
    println!("Hello, world!");

    let addr = std::env::args().nth(1).expect("Address:PORT");

    let chat_table = Arc::new(chats_map::ChatTracker::new());

    
//sec17_vid106: 18.45
    task::block_on(async {
        let listener = net::TcpListener::bind(&addr).await?;

        let mut new_connections = listener.incoming();
        while let Some(socket_result) = new_connections.next().await {
            let socket = socket_result?;
            let chats = chat_table.clone();

            task::spawn(async {
                log_error(handle(socket,chats).await);
            });
        }
        Ok(())
        // socket.set_nodelay(true)?;
        // let send = send(socket.clone());
        // let replies = messages(socket);
        // //send & replies should be run concurrently, and race each other to complete
        // // their tasks
        // replies.race(send).await?;

        // Ok(())
    })
}
