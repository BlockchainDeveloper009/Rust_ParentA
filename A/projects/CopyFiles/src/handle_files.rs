use std::fs;
use std::io;
use std::path::Path;

/// Copies missing files from folder A to folder C, based on files present in folder B.
///
/// # Arguments
/// * `folder_a` - A string slice that holds the path to the source folder (Folder A).
/// * `folder_b` - A string slice that holds the path to the comparison folder (Folder B).
/// * `folder_c` - A string slice that holds the path to the destination folder (Folder C).
///
/// # Returns
/// * `Ok(())` if operation is successful.
/// * `Err(io::Error)` if an error occurs.
pub fn copy_missing_files(folder_a: &str, folder_b: &str, folder_c: &str) -> io::Result<()> {
    // Create folder C if it doesn't exist
    fs::create_dir_all(folder_c)?;

    // Get list of files in folders A and B
    let files_in_a = get_files_in_folder(folder_a)?;
    let files_in_b = get_files_in_folder(folder_b)?;

    // Copy missing files from folder A to folder C
    for file in files_in_a {
        if !files_in_b.contains(&file) {
            let source_path = Path::new(folder_a).join(&file);
            let destination_path = Path::new(folder_c).join(&file);

            // Copy file from folder A to folder C
            fs::copy(source_path, destination_path)?;
            println!("Copied missing file: {}", file);
        }
    }

    Ok(())
}

/// Returns a list of file names in the specified folder.
///
/// # Arguments
/// * `folder` - A string slice that holds the path to the folder.
///
/// # Returns
/// * `Ok(Vec<String>)` with file names if operation is successful.
/// * `Err(io::Error)` if an error occurs.
pub fn get_files_in_folder(folder: &str) -> io::Result<Vec<String>> {
    let mut files = Vec::new();

    for entry in fs::read_dir(folder)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            if let Some(file_name) = path.file_name() {
                if let Some(file_name_str) = file_name.to_str() {
                    files.push(file_name_str.to_string());
                }
            }
        }
    }

    Ok(files)
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_files_in_folder() {
        let folder = r"C:\Users\krtzx\OneDrive\Pictures\Fin\2024\SirioFinance";
        match get_files_in_folder(folder) {
            Ok(files) => {
                println!("Files in folder:");
                for file in files {
                    println!("{}", file);
                }
            }
            Err(e) => eprintln!("Error getting files: {}", e),
        }
    }

    #[test]
    fn test_copy_missing_files() {
        let folder_a = r"C:\Users\krtzx\OneDrive\Pictures\Fin\2024\SirioFinance1";
        let folder_b = r"C:\Users\krtzx\OneDrive\Pictures\Fin\2024\B";
        let folder_c = r"C:\Users\krtzx\OneDrive\Pictures\Fin\2024\C";
        match copy_missing_files(folder_a, folder_b, folder_c) {
            Ok(_) => println!("Missing files copied successfully."),
            Err(e) => eprintln!("Error during file copy: {}", e),
        }
    }
}
pub fn test_copy_missing_files() {
    let folder_a = r"C:\Users\krtzx\OneDrive\Pictures\Fin\2024\SirioFinance";
    let folder_b = r"C:\Users\krtzx\OneDrive\Pictures\Fin\2024\B";
    let folder_c = r"C:\Users\krtzx\OneDrive\Pictures\Fin\2024\C";
    match copy_missing_files(folder_a, folder_b, folder_c) {
        Ok(_) => println!("Missing files copied successfully."),
        Err(e) => eprintln!("Error during file copy: {}", e),
    }
}