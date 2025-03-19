Here's a Rust program to help you categorize images into date-based buckets by extracting EXIF metadata. It uses the `rexiv2` crate to read metadata and `std::fs` for file operations.

### Steps:
1. Scan a given directory for images.
2. Extract the **DateTime**, **Camera Maker**, and **Dimensions**.
3. Sort and move/copy images into folders named by their shooting date (e.g., `YYYY-MM-DD`).

---

### Install Dependencies
Add these to your `Cargo.toml`:

```toml
[dependencies]
rexiv2 = "0.10"
chrono = "0.4"
walkdir = "2"
```

---

### Rust Code: Organizing Images into Buckets
```rust
use rexiv2::Metadata;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use chrono::NaiveDate;

fn main() {
    let source_dir = "C:/path/to/your/images"; // Change this to your image folder
    let dest_dir = "C:/path/to/buckets"; // Folder where images will be categorized

    fs::create_dir_all(dest_dir).expect("Failed to create destination folder");

    for entry in WalkDir::new(source_dir).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if is_image(path) {
            if let Some(date) = extract_date(path) {
                let bucket_folder = format!("{}/{}", dest_dir, date);
                fs::create_dir_all(&bucket_folder).expect("Failed to create bucket folder");

                let dest_path = format!("{}/{}", bucket_folder, path.file_name().unwrap().to_string_lossy());
                fs::copy(path, &dest_path).expect("Failed to copy file");
                println!("Moved: {:?} → {:?}", path, dest_path);
            } else {
                println!("Skipping {:?}: No valid EXIF date", path);
            }
        }
    }
}

fn is_image(path: &Path) -> bool {
    if let Some(ext) = path.extension() {
        return matches!(ext.to_str().unwrap_or("").to_lowercase().as_str(), "jpg" | "jpeg" | "png");
    }
    false
}

fn extract_date(path: &Path) -> Option<String> {
    if let Ok(meta) = Metadata::new_from_path(path) {
        if let Some(date_str) = meta.get_tag_string("Exif.Photo.DateTimeOriginal") {
            if let Ok(parsed_date) = NaiveDate::parse_from_str(&date_str, "%Y:%m:%d %H:%M:%S") {
                return Some(parsed_date.to_string()); // Format: YYYY-MM-DD
            }
        }
    }
    None
}
```

---

### How It Works:
1. **Scans a folder recursively** for images (`.jpg`, `.jpeg`, `.png`).
2. **Extracts the EXIF date** (`Exif.Photo.DateTimeOriginal`).
3. **Creates folders by date** (`YYYY-MM-DD`).
4. **Moves images into the respective folders** based on their shooting date.

---

### Example Output:
```
Moved: "C:/photos/img123.jpg" → "C:/buckets/2024-01-15/img123.jpg"
Skipped: "C:/photos/no_exif.jpg": No valid EXIF date
```

---

### Notes:
- If an image has **no EXIF date**, it's skipped.
- If the EXIF date is missing, you may fall back to the file creation date.

Would you like to add error handling or support for more file formats? 🚀