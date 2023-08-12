use std::fs;

use crate::cli::Cli;

use super::{HandlerImpl, HandlerError};

pub struct DotPartyHandler;

impl DotPartyHandler {
    pub fn new() -> Self {
        Self
    }
}

impl HandlerImpl for DotPartyHandler {
    fn handle(cli: &Cli) -> Result<(), HandlerError> {
        let path = &cli.path; // HandlerError::FailedToReadDir(path.to_string_lossy().to_string())
        let dir = match fs::read_dir(path) {
            Ok(dir) => dir,
            Err(_err) => {
                return Err(HandlerError::FailedToReadDir(path.to_string_lossy().to_string()))
            }
        };
        
        for entry in dir { // Possibly continue after failed one file
            if let Ok(entry) = entry {
                let filename = entry.file_name();
                let filename_str = filename.to_string_lossy();

                if !entry.metadata().map(|md| md.is_file()).unwrap_or(false) {
                    continue;
                }

                let new_folder = match filename_str.split('_').next() {
                    Some(new_folder) => new_folder,
                    None => {
                        return Err(HandlerError::InvalidFilename(filename_str.to_string()))
                    }
                };
                let new_folder_path = path.join(new_folder);

                let new_path = new_folder_path.join(&filename);
                if let Err(_) = fs::create_dir_all(&new_folder_path) {
                    return Err(HandlerError::FailedToCreateDir(new_folder_path.to_string_lossy().to_string()));
                }
                if let Err(_) = fs::rename(path.join(&*filename_str), &new_path) {
                    return Err(HandlerError::FailedToMoveDir(path.join(&*filename_str).to_string_lossy().to_string(), new_path.to_string_lossy().to_string()));
                }
                
                // if fs::metadata(&new_folder_path).is_ok() {
                //     let new_path = new_folder_path.join(&filename);
                //     fs::rename(path.join(&*filename_str), &new_path).expect("Failed to rename file");
                // } else {
                //     fs::create_dir_all(&new_folder_path).expect("Failed to create directory");
                //     let new_path = new_folder_path.join(&filename);
                //     fs::rename(path.join(&*filename_str), &new_path).expect("Failed to rename file");
                // }
            }
        }

        Ok(())
    }
}