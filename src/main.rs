use std::fs;
use std::path::{Path, PathBuf};
use std::collections::HashMap;
use std::io;

struct FileOrganizer {
    source_dir: PathBuf,
    organized_files: HashMap<String, Vec<String>>,
}

impl FileOrganizer {
    fn new(path: &str) -> io::Result<FileOrganizer> {
        let source_dir = PathBuf::from(path);
        if !source_dir.exists() {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "Source directory does not exist",
            ));
        }
        Ok(FileOrganizer {
            source_dir,
            organized_files: HashMap::new(),
        })
    }

    fn organize(&mut self) -> io::Result<()> {
        // Read directory contents
        let entries = fs::read_dir(&self.source_dir)?;

        for entry in entries {
            let entry = entry?;
            let path = entry.path();

            // Skip if it's a directory
            if path.is_dir() {
                continue;
            }

            // Get file extension
            let extension = path
                .extension()
                .and_then(|ext| ext.to_str())
                .unwrap_or("unknown")
                .to_lowercase();

            // Create destination directory if it doesn't exist
            let dest_dir = self.source_dir.join(&extension);
            if !dest_dir.exists() {
                fs::create_dir(&dest_dir)?;
            }

            // Move file to its corresponding directory
            let file_name = path.file_name().unwrap().to_string_lossy().to_string();
            let new_path = dest_dir.join(&file_name);
            fs::rename(&path, &new_path)?;

            // Store the organized file information
            self.organized_files
                .entry(extension)
                .or_insert_with(Vec::new)
                .push(file_name);
        }

        Ok(())
    }

    fn print_summary(&self) {
        println!("\nFile Organization Summary:");
        println!("-------------------------");
        
        for (extension, files) in &self.organized_files {
            println!("\n.{} files:", extension);
            for file in files {
                println!("  - {}", file);
            }
        }
    }
}

fn main() {
    println!("Welcome to File Organizer!");
    println!("Enter the directory path to organize: ");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let path = input.trim();

    match FileOrganizer::new(path) {
        Ok(mut organizer) => {
            println!("Organizing files in: {}", path);
            
            match organizer.organize() {
                Ok(_) => {
                    println!("\nFiles organized successfully!");
                    organizer.print_summary();
                }
                Err(e) => println!("Error organizing files: {}", e),
            }
        }
        Err(e) => println!("Error creating organizer: {}", e),
    }
}