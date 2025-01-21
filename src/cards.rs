use std::error::Error;
use std::fs::{self};
use std::path::PathBuf;
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
    _ = Command::new("xdg-open").arg(&file_path).spawn(); // Use spawn to run the command asynchronously
}

#[cfg(test)]
mod test {
    use super::open_file;
    #[test]
    fn test_img() {
        open_file("/home/igor/code/rust/Flash_Cards/cards/1029684.jpg".to_string());
    }
}
