use crate::models::CopyFiles_Params_test::CopyFiles_Params_test;

//use crate::models::CopyFiles_Params_test;
use std::fs;
use std::io;
use std::path::Path;
use UtilsB::helpers::filehelper::{ut_fhlprs_read_file, ut_fhlprs_write_file, ut_flhprs_print};

// use UtilsB::helpers::filehelper::{fhlprs_read_file};
// import filehelper module from UtilsB library
// pub mod helper;
// use helper::read_file;

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
///
pub fn copy_missing_files(_CopyFiles_Params_test: CopyFiles_Params_test) -> io::Result<()> {
    let mut folder_a: &String = &_CopyFiles_Params_test.sourceFolderA;
    let mut folder_b: &String = &_CopyFiles_Params_test.sourceFolderB;
    let mut folder_c: &String = &_CopyFiles_Params_test.destinationFolder;
    println!("folder_c ==========>>>>>> {}", folder_c);
    // Create folder C if it doesn't exist
    fs::create_dir_all(folder_c)?;
    let mut filesBeingCopied: Vec<String> = Vec::new();

    let mut fileAlreadyExists: Vec<String> = Vec::new();
    // Get list of files in folders A and B
    let files_in_a = get_files_in_folder(folder_a)?;
    let files_in_b = get_files_in_folder(folder_b)?;

    // Copy missing files from folder A to folder C
    for file in files_in_a {
        if !files_in_b.contains(&file) {
            let source_path = Path::new(&_CopyFiles_Params_test.sourceFolderA).join(&file);
            let destination_path = Path::new(&_CopyFiles_Params_test.destinationFolder).join(&file);

            // Copy file from folder A to folder C

            if (_CopyFiles_Params_test.copyFiles_flag) {
                fs::copy(source_path, destination_path)?;
                println!("Copied missing file: {}", file);
            } else {
                //filesBeingCopied.push(&source_path);
                filesBeingCopied.push(source_path.display().to_string());
                //source_path
            }
        } else {
            ut_flhprs_print("add to a vector on existing files");

            fileAlreadyExists.push(file);
            ut_flhprs_print(&fileAlreadyExists.join("|"));
        }
    }

    ut_fhlprs_write_file("Copied", &filesBeingCopied.join("|\n"));
    ut_fhlprs_write_file("Exists", &fileAlreadyExists.join(",\n"));
    //write your file aready existn
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

    let path = Path::new(folder);
    // Check if the folder exists and is a directory
    if !path.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Folder does not exist",
        ));
    }
    if !path.is_dir() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Path is not a directory",
        ));
    }

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
    fn test_copy_missing_files_paramsFile() {
        let folder_a = r"C:\Users\krtzx\OneDrive\Pictures\Fin\2024\SirioFinance1";
        let folder_b = r"C:\Users\krtzx\OneDrive\Pictures\Fin\2024\B";
        let folder_c = r"C:\Users\krtzx\OneDrive\Pictures\Fin\2024\C";
        //let param = read params.json;
        // let param = get_params();
        match copy_missing_files(folder_a, folder_b, folder_c) {
            Ok(_) => println!("Missing files copied successfully."),
            Err(e) => eprintln!("Error during file copy: {}", e),
        }
    }
    #[test]
    pub fn test_copy_missing_utilHelper_file_read() {
        let folder_a = r"C:\Users\krtzx\OneDrive\Pictures\Fin\2024\SirioFinance";
        let folder_b = r"C:\Users\krtzx\OneDrive\Pictures\Fin\2024\B";
        let folder_c = r"C:\Users\krtzx\OneDrive\Pictures\Fin\2024\C";
        println!("----test1------");
        let param = ut_fhlprs_read_file("params.json");
        println!("-- {}", folder_b);
        println!("param: {:?}", param);
        match copy_missing_files(folder_a, folder_b, folder_c) {
            Ok(_) => println!("Missing files copied successfully."),
            Err(e) => eprintln!("Error during file copy: {}", e),
        }
    }
}
