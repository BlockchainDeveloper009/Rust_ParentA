use std::fs::File;
use std::io::Read; // Needed to read file contents
use std::path::PathBuf;
use serde_json::Value; // Needed to parse JSON

pub struct Params {
    pub port: u16,
    pub host: String,
    pub db_host: String,
    pub db_port: u16,
    pub db_user: String,
    pub db_password: String,
    pub db_name: String,
}

// Get the params from the config file
pub fn get_params() -> Params {

    /* Why CARGO_MANIFEST_DIR?
env!("CARGO_MANIFEST_DIR") gets the root directory of the current package (ApiServer).
PathBuf::from(env!("CARGO_MANIFEST_DIR")) ensures we always look inside the correct project (ApiServer).
path.push("src/config.json") ensures we access config.json inside ApiServer/src/.
3️⃣ What This Solves
✅ Works regardless of where cargo run is executed.
✅ Keeps config.json inside ApiServer, independent of the CLI (A).
✅ Prevents hardcoded paths and makes your project portable.
 */
       // Get the ApiServer project root dynamically
       let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
       path.push("src/config.json"); // Path relative to ApiServer's root
   
       println!("📂 Looking for config file at: {:?}", path); // Debug print

    let mut file = File::open(path).expect("file not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("something went wrong reading the file");

    let v: Value = serde_json::from_str(&contents).expect("JSON was not well-formatted");

    Params {
        port: v["port"].as_u64().unwrap_or(8000) as u16,  // Default: 8000
        host: v["host"].as_str().unwrap_or("localhost").to_string(),
        db_host: v["db_host"].as_str().unwrap_or("localhost").to_string(),
        db_port: v["db_port"].as_u64().unwrap_or(5432) as u16,
        db_user: v["db_user"].as_str().unwrap_or("postgres").to_string(),
        db_password: v["db_password"].as_str().unwrap_or("password").to_string(),
        db_name: v["db_name"].as_str().unwrap_or("rustaceans").to_string(),
    }
}
