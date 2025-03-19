//use my_crate::handle_files; // Replace `my_crate` with your actual crate name
//use rexiv2::Metadata;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use chrono::NaiveDate;

use std::fs::File;

use rexif::parse_buffer;
use std::io::{Read, BufReader}; // Add Read to import

// #[test]
// fn test_copy_missing_files() {
//     let result = handle_files::copy_missing_files("src", "dest");
//     assert!(result.is_ok());
// }

#[test]
fn test_Dates() {

    let date_str = "2025:08:19 12:40:28";
    let parsed_date = NaiveDate::parse_from_str(&date_str, "%Y:%m:%d %H:%M:%S");
    assert!(parsed_date.is_ok());
    assert_eq!(parsed_date.unwrap().to_string(), "2023-08-19");
    let fut_date_str = "2023:08:28 12:40:28";

    if(date_str < fut_date_str) {
        println!("date_str is less than fut_date_str");
    } else {
        println!("date_str is greater than fut_date_str");
    }


    if(fut_date_str < date_str ) {
        println!("date_str is less than fut_date_str");
    } else {
        println!("date_str is greater than fut_date_str");
    }
    // compare = parsed_date.cmp(&fut_date_str);
    // assert_eq!(compare, Ordering::Less);
 }

#[test]
fn test_Reading_MetaDataForSingleImage() { 
    let file_path = r"C:\Users\krtzx\OneDrive\Pictures\Camera Roll\2023_july_and_later\20230819_124028.jpg"; // Change this to your image folder
    let file = File::open(file_path).expect("Failed to open file");
    let mut bufreader = BufReader::new(file);
    let mut buffer = Vec::new();
    
    bufreader.read_to_end(&mut buffer).expect("Failed to read file");

    match parse_buffer(&buffer) {
        Ok(exif_data) => {
            assert!(exif_data.entries.len() > 0, "No EXIF data found!");
                println!("-- -----");
            for entry in exif_data.entries {

               // dbg!(&entry.tag, &entry.value_more_readable);

                println!("-- values {}: {}", entry.tag, entry.value_more_readable);
                // println!("-- values {}", entry.to_string());
               

            }
        }
        Err(e) => {
            eprintln!("Error reading EXIF data: {:?}", e);
        }
    }

}
#[test]
fn test_OrganizeBuckets() {
    let source_dir = r"C:\Users\krtzx\OneDrive\Pictures\Camera Roll\2023_july_and_later"; // Change this to your image folder
    let dest_dir = r"C:\Users\krtzx\OneDrive\Pictures\Camera Roll\copied"; // Folder where images will be categorized

    // fs::create_dir_all(dest_dir).expect("Failed to create destination folder");

    // for entry in WalkDir::new(source_dir).into_iter().filter_map(|e| e.ok()) {
    //     let path = entry.path();
    //     if is_image(path) {
    //         if let Some(date) = extract_date(path) {
    //             let bucket_folder = format!("{}/{}", dest_dir, date);
    //             fs::create_dir_all(&bucket_folder).expect("Failed to create bucket folder");

    //             let dest_path = format!("{}/{}", bucket_folder, path.file_name().unwrap().to_string_lossy());
    //             fs::copy(path, &dest_path).expect("Failed to copy file");
    //             println!("Moved: {:?} → {:?}", path, dest_path);
    //         } else {
    //             println!("Skipping {:?}: No valid EXIF date", path);
    //         }
    //     }
    // }
}



// fn is_image(path: &Path) -> bool {
//     if let Some(ext) = path.extension() {
//         return matches!(ext.to_str().unwrap_or("").to_lowercase().as_str(), "jpg" | "jpeg" | "png");
//     }
//     false
// }

// fn extract_date(path: &Path) -> Option<String> {
//     if let Ok(meta) = Metadata::new_from_path(path) {
//         if let Some(date_str) = meta.get_tag_string("Exif.Photo.DateTimeOriginal") {
//             if let Ok(parsed_date) = NaiveDate::parse_from_str(&date_str, "%Y:%m:%d %H:%M:%S") {
//                 return Some(parsed_date.to_string()); // Format: YYYY-MM-DD
//             }
//         }
//     }
//     None
// }
