use std::task::{Poll, Context};
use async_std::{fs::File, io, prelude::*, task};

trait Future {
    type Output;
    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut Context<'_>) 
    -> Poll<Self::Output>;
}

async fn async_read_file(file_path: &str) -> io::Result<String> {
    let mut file = File::open(file_path).await?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).await?;
    Ok(contents)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_Async_Read_File() {



        let task = task::spawn(async {

            let result = async_read_file("dsaParams.json").await;
            match result {
                Ok(k) => println!("File content: {}", k),
                Err(e) => eprintln!("Error reading file: {}", e),
            }

        });

        

        println("Task has started!");
        task::block_on(task);
        println("Task has completed!");

    }
    #[test]
    fn test_task() {
        async_read_file();

    }
}