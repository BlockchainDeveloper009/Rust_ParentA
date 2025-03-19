use chrono::Local;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::Path;

fn main_log_mod() {
    // Generate a timestamp
    let timestamp = Local::now().format("%Y-%m-%d_%H-%M-%S").to_string();
    
    // Extract filename from a given path
    let file_path = "src/main.rs"; // Change this to your file path
    let file_name = Path::new(file_path)
        .file_name()
        .unwrap_or_default()
        .to_str()
        .unwrap_or("unknown");

    // Create log directory
    let log_dir = "logs";
    if !Path::new(log_dir).exists() {
        fs::create_dir(log_dir).expect("Failed to create log directory");
    }

    // Create a log file with timestamp
    let log_file_path = format!("{}/{}_{}.log", log_dir, file_name, timestamp);
    let mut log_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&log_file_path)
        .expect("Failed to open log file");

    // Write log content
    let log_content = format!("[{}] Program started. Logging to {}\n", timestamp, log_file_path);
    log_file
        .write_all(log_content.as_bytes())
        .expect("Failed to write to log file");

    println!("Log written to: {}", log_file_path);
}
