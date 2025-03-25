use async_std::prelude::*;

use async_std::io::BufRead;
use serde::de::DeserializeOwned;
use async_std::prelude::*;
use serde::Serialize;
use std::error::Error;
use std::marker::Unpin;

use async_std::io::Write;

// type alias is used as error for entire lifetime of the program
pub type ChatError = Box<dyn Error + Send + Sync + 'static>;
pub type ChatResult<T> = Result<T, ChatError>;


pub async fn send_json<O,P>(leaving: &mut O, packet: &P) -> ChatResult<()>
where
    O: Write + Unpin, // O must implement Write trait and Unpin
    P: Serialize,  // P must implement Serialize trait
{
    let mut json = serde_json::to_string(&packet)?;
    json.push('\n');

    leaving.write_all(json.as_bytes()).await?;
   // leaving.write_all(b"\n").await?;
   // leaving.flush().await?;
    Ok(())
}

pub fn receive <I, T>(incoming: I) -> impl Stream<Item = ChatResult<T>>
where
    I: BufRead + Unpin,
    T: DeserializeOwned, // T must implement DeserializeOwned trait SEC17: vid.105: 13.56 sec
{


    incoming.lines().map(|line| -> ChatResult<T> {

        let li = line?;
        let msg = serde_json::from_str::<T>(&li)?;
        Ok(msg)
    })

    // async_stream::stream! {
    //     let mut buf = String::new();
    //     let mut incoming = BufReader::new(incoming);

    //     loop {
    //         buf.clear();
    //         match incoming.read_line(&mut buf).await {
    //             Ok(0) => break,
    //             Ok(_) => {
    //                 let packet: T = serde_json::from_str(&buf)?;
    //                 yield Ok(packet);
    //             }
    //             Err(e) => {
    //                 yield Err(e.into());
    //                 break;
    //             }
    //         }
    //     }
    // }


}
//dropbox is for everyone
//not for everyone blockchain
//i like high performance computing systems
// challenging systems engineering
// i happen to talk to HR to find the work culture, 
// tech professionals mind change, background, study background
// i like to stay and work with top notch professionals in blockchain industry

// help people to grow
// i like to work with people who are passionate about their work
// i helped my manager to achieve things, he got promoted, achieve this goal and vision for the eyar
