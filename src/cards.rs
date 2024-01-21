use std::error::Error;
use std::fs::{self};
use std::path::{PathBuf};
use std::process::Command;

pub fn get_files(path: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let entries = fs::read_dir(path)?;

    let file_paths: Vec<String> = entries
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path: PathBuf = entry.path();
            Some(path.to_string_lossy().into_owned())
        })
        .collect();

    Ok(file_paths)
}

pub fn open_file(file_path: String) {

    #[cfg(target_os = "macos")]
        {
            Command::new("open")
                .arg(file_path)
                .spawn()
                .expect("Failed to open file");
        }
    
}


/* 
 let file_path = "path/to/your/file.txt";

    // Open the file
    let _file = File::open(file_path)?;

    // If you only want to open the file without doing anything with it in Rust
    // and let the operating system open it with the default application,
    // you can use the `open` command on macOS.
    #[cfg(target_os = "macos")]
    {
        Command::new("open")
            .arg(file_path)
            .spawn()
            .expect("Failed to open file");
    }

    Ok(())
 * */
