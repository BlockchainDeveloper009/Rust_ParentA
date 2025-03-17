use chrono::Local;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::Path;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_create_log() {
        
        let result = write_log("unit test call from logger.rs file ");
        assert_eq!(result, ());
    }
}

pub fn write_log(message: &str) {
    // Generate timestamp
    let timestamp = Local::now().format("%Y-%m-%d_%H-%M-%S").to_string();

    // Define log directory
    let log_dir = "logs";
    if !Path::new(log_dir).exists() {
        println!("Creating log directory: {}", log_dir);
        fs::create_dir(log_dir).expect("Failed to create log directory");
    }

    // Define log file path
    let log_file_path = format!("{}/log_{}.log", log_dir, timestamp);

    // Open or create log file
    let mut log_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&log_file_path)
        .expect("Failed to open log file");

    // Write message to log file
    let log_content = format!("[{}] : {}\n", timestamp, message);
    log_file
        .write_all(log_content.as_bytes())
        .expect("Failed to write to log file");

    println!("Log written to: {}", log_file_path);
}
