mod handle_files; // If not already declared
use crate::handle_files::{copy_missing_files, get_files_in_folder};


use std::io;
use std::fs;
fn main() {
    println!("Hello, from CopyFiles main.rs!");

    println!("Choose an operation:");
    println!("1. Database sample operation");
    println!("2. Copy missing files");

    let mut input = String::new();
    io::stdin().read_line(&mut input);
    let choice = input.trim();

    match choice {
        "1" => {
            // Call the database operation
            match call_db() {
                _ => todo!(),
                // Ok(_) => println!("Database operation completed successfully."),
                // Err(e) => eprintln!("Error during database operation: {}", e),
                // ()
            }
        }
        "2" => {
            // Define your folder paths
            let folder_a = r"C:\Users\krtzx\OneDrive\Pictures\Fin\2024\SirioFinance";
            let folder_b = r"C:\Users\krtzx\OneDrive\Pictures\Fin\2024\B";
            let folder_c = r"C:\Users\krtzx\OneDrive\Pictures\Fin\2024\C";

            // Call the file copying function
            match copy_missing_files(folder_a, folder_b, folder_c) {
                Ok(_) => println!("Missing files copied successfully."),
                Err(e) => eprintln!("Error during file copy: {}", e),
            }
        }
        _ => println!("Invalid choice. Please choose either 1 or 2."),
    }

    //Ok(())
}
pub fn call_db(){
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