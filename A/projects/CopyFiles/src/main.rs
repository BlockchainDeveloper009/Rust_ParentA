mod models;

use crate::models::CopyFiles_Params_test::CopyFiles_Params_test;

mod handle_files; // If not already declared
use crate::handle_files::{copy_missing_files, get_files_in_folder};

use serde::{Deserialize, Serialize};
use UtilsB::helpers::filehelper::ut_fhlprs_read_file;
use UtilsB::log_mod::logger;

use std::fs;
use std::io;
fn main() {
    println!("Hello, from CopyFiles main.rs!");
    let fp = r"C:\source\repos\Rust_ParentA\A\projects\E\configs\CopyFiles_Params_test.json";
    let file_content = ut_fhlprs_read_file(fp).unwrap();

    // Deserialize into CopyFiles_Params_test struct
    let param_json =
        serde_json::from_str::<CopyFiles_Params_test>(&file_content).expect("Failed to parse JSON");

    println!(
        "file_content:  sourceFolderA==>  {}",
        param_json.sourceFolderA
    );
    // println!("Hello from the Utils library! {}", file_content);
    println!("calling copy_missing_files--------");
    copy_missing_files(param_json).expect("Failed to copy missing file");

    println!("AFTERRRR__calling copy_missing_files--------");
    // println!("Choose an operation:");
    // println!("1. Database sample operation");
    // println!("2. Copy missing files");
    // println!("_ or any key for exit");

    // let mut input = String::new();
    // io::stdin().read_line(&mut input);
    // let choice = input.trim();

    // match choice {
    //     "1" => {
    //         // Call the database operation
    //         match call_db() {
    //             _ => todo!(),
    //             // Ok(_) => println!("Database operation completed successfully."),
    //             // Err(e) => eprintln!("Error during database operation: {}", e),
    //             // ()
    //         }
    //     }
    //     "2" => {
    //         // Define your folder paths
    //         let folder_a = r"C:\Users\krtzx\OneDrive\Pictures\Fin\2024\SirioFinance";
    //         let folder_b = r"C:\Users\krtzx\OneDrive\Pictures\Fin\2024\B";
    //         let folder_c = r"C:\Users\krtzx\OneDrive\Pictures\Fin\2024\C";

    //         // Call the file copying function
    //         match copy_missing_files(folder_a, folder_b, folder_c) {
    //             Ok(_) => println!("Missing files copied successfully."),
    //             Err(e) => eprintln!("Error during file copy: {}", e),
    //         }
    //     }
    //     _ => {
    //         println!("=====================================");
    //         println!("Thank you, You dint press valid entries 1 or 2, so program exits");
    //         println!("=====================================");
    //     }
    // }

    //Ok(())
}
pub fn call_db() {
    // // Define your database connection parameters
    // let params = get_params();

    // // Connect to the database
    // let conn = match connect_to_db(&params) {
    //     Ok(conn) => conn,
    //     Err(e) => return Err(e),
    // };

    // // Perform database operations
    // match perform_db_operations(&conn) {
    //     Ok(_) => println!("Database operations completed successfully."),
    //     Err(e) => return Err(e),
    // };

    // // Close the database connection
    // match close_db_connection(&conn) {
    //     Ok(_) => println!("Database connection closed successfully."),
    //     Err(e) => return Err(e),
    // };

    // Ok(())
}
