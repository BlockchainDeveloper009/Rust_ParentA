// Add this to your Cargo.toml
// [dependencies]
// tokio = { version = "1.0", features = ["full", "macros"] }
// serde = { version = "1.0", features = ["derive"] }
// serde_json = "1.0"

use crate::models::CopyFiles_Params_test::CopyFiles_Params_test;
use UtilsB::helpers::filehelper::{ut_fhlprs_read_file, ut_fhlprs_write_file, ut_flhprs_print};
use std::path::Path;
use tokio::fs as async_fs;
use std::io;
pub fn load_params_from_file(file_path: &str) -> CopyFiles_Params_test {
    let file_content = ut_fhlprs_read_file(file_path).unwrap();
    serde_json::from_str::<CopyFiles_Params_test>(&file_content).expect("Failed to parse JSON")
}
// Improved async version with proper error handling
pub async fn copy_missing_files_async(params: CopyFiles_Params_test) -> io::Result<usize> {
    let folder_a = &params.sourceFolderA;
    let folder_b = &params.sourceFolderB;
    let folder_c = &params.destinationFolder;
    
    println!("[DEBUG] Destination folder: {}", folder_c);

    // Create folder C if it doesn't exist (async version)
    async_fs::create_dir_all(folder_c).await?;

    let (files_in_a, files_in_b, files_in_c) = tokio::join!(
        get_files_in_folder_async(folder_a),
        get_files_in_folder_async(folder_b),
        get_files_in_folder_async(folder_c)
    );

    let files_in_a = files_in_a?;
    let files_in_b = files_in_b?;
    let files_in_c = files_in_c?;

    println!("[DEBUG] Files in A: {}", files_in_a.join(", "));
    println!("[DEBUG] Files in B: {}", files_in_b.join(", "));
    println!("[DEBUG] Files in C: {}", files_in_c.join(", "));

    let mut copied_files = Vec::new();
    let mut existing_files = Vec::new();

    for file in files_in_a {
        if !files_in_b.contains(&file) && !files_in_c.contains(&file) {
            let source_path = Path::new(folder_a).join(&file);
            let dest_path = Path::new(folder_c).join(&file);

            if params.copyFiles_flag {
                // Async file copy
                async_fs::copy(&source_path, &dest_path).await?;
                println!("[SUCCESS] Copied: {}", file);
                copied_files.push(source_path.display().to_string());
            } else {
                println!("[DRY RUN] Would copy: {}", file);
                copied_files.push(source_path.display().to_string());
            }
        } else {
            existing_files.push(file.clone());
            println!("[SKIPPED] File exists: {}", file);
        }
    }

    // Async file writing
    if !copied_files.is_empty() {
        async_fs::write("Copied.txt", copied_files.join("\n")).await?;
    }
    if !existing_files.is_empty() {
        async_fs::write("Exists.txt", existing_files.join("\n")).await?;
    }

    Ok(copied_files.len())
}

// Improved async directory listing
pub async fn get_files_in_folder_async(folder: &str) -> io::Result<Vec<String>> {
    let mut entries = async_fs::read_dir(folder).await?;
    let mut files = Vec::new();

    while let Some(entry) = entries.next_entry().await? {
        let path = entry.path();
        if path.is_file() {
            if let Some(file_name) = path.file_name() {
                files.push(file_name.to_string_lossy().into_owned());
            }
        }
    }

    Ok(files)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    #[tokio::test]
    async fn test_2_async_file1_gpt() -> io::Result<()> {
        println!("[START] ASYNC file copy test");
        
        let params = load_params_from_file(
            r"C:\source\repos\Rust_ParentA\A\projects\E\configs\CopyFiles_Params_test1_async.json"
        );

        match copy_missing_files_async(params).await {
            Ok(count) => {
                println!("Copied {} files successfully", count);
                assert!(count > 0, "No files were copied");
            }
            Err(e) => panic!("Test failed: {}", e),
        }

        println!("[END] Test completed");
        Ok(())
    }

    // Add tempdir cleanup example
    #[tokio::test]
    async fn test_with_temp_dirs() -> io::Result<()> {
        use tempfile::tempdir;

        let dir_a = tempdir()?;
        let dir_b = tempdir()?;
        let dir_c = tempdir()?;

        // Create test files
        async_fs::write(dir_a.path().join("test1.txt"), "content").await?;
        async_fs::write(dir_b.path().join("test2.txt"), "content").await?;

        let params = CopyFiles_Params_test {
            SourceFoldersCount: 1,
            SourceFoldersCountPathFlag : false,
            masterFilePath: "".to_string(),
            sourceFolderA: dir_a.path().to_str().unwrap().to_string(),
            sourceFolderB: dir_b.path().to_str().unwrap().to_string(),
            destinationFolder: dir_c.path().to_str().unwrap().to_string(),
            copyFiles_flag: true,
            writeToMasterPathFlag: false,
        };

        let result = copy_missing_files_async(params).await?;
        assert_eq!(result, 1); // Only test1.txt should be copied
        
        Ok(())
    }
}
