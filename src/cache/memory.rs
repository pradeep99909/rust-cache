use std::fs::{self, File, OpenOptions};
use std::io::{self, Write};
use std::path::Path;
use std::time;

pub struct Storage {
    file_path: String,
}

impl Storage {
    pub fn new(file_path: &str) -> Self {
        let path = Path::new(file_path);

        // Create the file if it doesn't exist
        if !path.exists() {
            File::create(path).expect("Failed to create cache.txt");
        }

        Self {
            file_path: file_path.to_string(),
        }
    }

    fn write_file(&self, operation: &str, key: &str, contents: &str) -> io::Result<()> {
        // Temporarily make it writable if it's read-only
        self.set_file_readonly(false)?;

        let mut file = OpenOptions::new()
            .append(true)
            .write(true)
            .open(&self.file_path)?;
        // concat operation, key and contents
        let log = format!("{} {} {}", operation, key, contents);
        file.write(
            (time::SystemTime::now()
                .duration_since(time::UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs()
                .to_string()
                + " "
                + &log)
                .as_bytes(),
        )?;
        file.write(b"\n")?;

        // Set it back to read-only after writing
        self.set_file_readonly(true)?;

        Ok(())
    }

    fn set_file_readonly(&self, readonly: bool) -> io::Result<()> {
        let mut perms = fs::metadata(&self.file_path)?.permissions();
        perms.set_readonly(readonly);
        fs::set_permissions(&self.file_path, perms)?;
        Ok(())
    }

    pub fn add(&self, operation: &str, key: &str, val: &str) {
        if let Err(e) = self.write_file(operation, key, val) {
            eprintln!("Error writing to file: {}", e);
        }
    }
}
