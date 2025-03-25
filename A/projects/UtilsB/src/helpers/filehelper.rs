use serde_json::Value;
use std::fs::File;
use std::io::{Read, Write}; // Needed to read file contents
use std::path::{Path, PathBuf}; // Needed to parse JSON

use chrono::Local;
use std::fs::{self, OpenOptions};

pub fn ut_flhprs_print(printMsg: &str) {
    println!("----- {} -----", printMsg);
}
pub fn ut_fhlprs_read_file(file_path: &str) -> Result<String, std::io::Error> {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push(file_path); // Path relative to ApiServer's root

    println!("📂 Looking for config file at: {:?}", path); // Debug print

    let mut file = File::open(path).expect("file not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    let v: Value = serde_json::from_str(&contents).expect("JSON was not well-formatted");

    Ok(contents)
}

pub fn ut_fhlprs_write_file(fname: &str, message: &str) {
    // Generate timestamp
    let timestamp = Local::now().format("%Y-%m-%d_%H-%M-%S").to_string();

    // Define log directory
    let log_dir = "logs";
    if !Path::new(log_dir).exists() {
        println!("Creating log directory: {}", log_dir);
        fs::create_dir(log_dir).expect("Failed to create log directory");
    }

    // Define log file path
    let log_file_path = format!("{}/{}_CopyFiles_log_{}.log", log_dir, timestamp, fname);
    println!("log file whole path {}, ", log_file_path);
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
