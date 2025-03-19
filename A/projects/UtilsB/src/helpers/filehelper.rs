use std::fs::File;
use std::io::Read; // Needed to read file contents
use std::path::PathBuf;
use serde_json::Value; // Needed to parse JSON


pub fn fhlprs_read_file(file_path: &str) -> Result<String, std::io::Error> {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push(file_path); // Path relative to ApiServer's root

    println!("📂 Looking for config file at: {:?}", path); // Debug print

 let mut file = File::open(path).expect("file not found");
 let mut contents = String::new();
 file.read_to_string(&mut contents).expect("something went wrong reading the file");

 let v: Value = serde_json::from_str(&contents).expect("JSON was not well-formatted");
    
    Ok(contents)
    

}