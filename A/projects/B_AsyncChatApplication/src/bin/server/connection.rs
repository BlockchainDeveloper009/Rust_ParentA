use async_std::io::BufReader;
use async_std::net::TcpStream;
use async_std::prelude::*;
use async_std::sync
use chat::utils::{self, ChatResult};
use chat::{Client, Server};
use crate::chats_map::ChatTracker;

pub struct Leaving(Mutex<TcpStream>);

impl Leaving {
    pub fn new(client: TcpStream) -> Self {
        Self(Mutex::new(client))
    }

    pub async fn send(&self, packet: Server) -> ChatResult<()> {
        let mut lock = self.0.lock().await;

        utils::send_json(&mut *lock, &packet).await?;
        lock.flush().await?;
        Ok(())
    }
}
// handling all connections from client side
pub async fn handle(socket: TcpStream, chats: Arc<ChatTracker>) -> ChatResult<()> {
    let leaving = Arc::new(Leaving::new(socket.clone()));

    let buffered = BufReader::new(socket);
    let mut from_client = utils::receive(buffered);  

    while le Some(req_res) = from_client.next().await {
        let request = req_res?;

        let result = match request {
            Client::Join {chat_name} => {
                let chat = chats.find_or_new(chat_name);
                chat.join(leaving.clone());
                Ok(())

            },
            Client::Post {chat_name, message} => match chats.find(&chat_name){
                Some(chat) => {
                    chat.post(message);
                    Ok(())
                }
                None => Err(format!("Chat does not exist: {}", chat_name)),
            },
        };

        if let Err(message) = result {
            let report = Server::Error(message);
            leaving.send(report).await?;
        }
    }
    Ok(())

}
pub async fn handle_copilot_generated(socket: TcpStream, chats: Arc<ChatTracker>) -> ChatResult<()> {
    let (reader, mut writer) = (&socket, &socket);
    let mut reader = BufReader::new(reader).lines();

    let mut chat_name = None;

    while let Some(line) = reader.next().await {
        let line = line?;
        let client = match line.trim() {
            "/join" => {
                let chat_name = get_chat_name(&line);
                chat_name = Some(chat_name.clone());
                Client::Join { chat_name }
            }
            _ => {
                let message = Arc::new(line);
                Client::Post {
                    chat_name: chat_name.clone().unwrap_or_else(|| Arc::new("main".to_string())),
                    message,
                }
            }
        };

        match client {
            Client::Join { chat_name } => {
                let chat_name = chat_name.clone();
                let mut chats = chats.lock().await;
                chats.add_chat(chat_name.clone());
                writer.write_all(format!("Joined chat: {}\n", chat_name).as_bytes()).await?;
            }
            Client::Post { chat_name, message } => {
                let chat_name = chat_name.clone();
                let message = message.clone();
                let mut chats = chats.lock().await;
                let packet = Server::Message { chat_name, message };
                chats.send_to_all(packet).await?;
            }
        }
    }

    Ok(())
}