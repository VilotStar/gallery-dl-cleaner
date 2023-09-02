use std::{fs, path::PathBuf, process::Command};

use crate::cli::{self, Cli};

use super::{HandlerError, HandlerImpl};

pub struct DotPartyHandler {
    path: PathBuf,
    update: Option<String>
}

impl DotPartyHandler {
    fn move_files_back(&self) -> Result<(), HandlerError> {
        let path = &self.path;
        let dir = match fs::read_dir(path) {
            Ok(dir) => dir,
            Err(_err) => {
                return Err(HandlerError::FailedToReadDir(
                    path.to_string_lossy().to_string(),
                ))
            }
        };

        for entry in dir {
            if let Ok(entry) = entry {
                let filename = entry.file_name();

                if !entry.metadata().map(|md| md.is_dir()).unwrap_or(false) {
                    continue;
                }

                let cur_dir_path = path.join(filename);
                let cur_dir = match fs::read_dir(&cur_dir_path) {
                    Ok(dir) => dir,
                    Err(_err) => {
                        return Err(HandlerError::FailedToReadDir(
                            path.to_string_lossy().to_string(),
                        ))
                    }
                };

                for entry in cur_dir {
                    if let Ok(entry) = entry {
                        if !entry.metadata().map(|md| md.is_file()).unwrap_or(false) {
                            continue;
                        }

                        if let Err(_) = fs::rename(
                            cur_dir_path.join(&entry.file_name()),
                            path.join(&entry.file_name()),
                        ) {
                            return Err(HandlerError::FailedToMoveDir(
                                cur_dir_path
                                    .join(&entry.file_name())
                                    .to_string_lossy()
                                    .to_string(),
                                path.join(&entry.file_name()).to_string_lossy().to_string(),
                            ));
                        }
                    }
                }
            }
        }

        Ok(())
    }
}

impl HandlerImpl for DotPartyHandler {
    fn new(path: &PathBuf, update: Option<String>) -> Self {
        Self { path: path.clone(), update }
    }

    fn handle(&self) -> Result<(), HandlerError> {
        let path = &self.path; // HandlerError::FailedToReadDir(path.to_string_lossy().to_string())
        let dir = match fs::read_dir(path) {
            Ok(dir) => dir,
            Err(_err) => {
                return Err(HandlerError::FailedToReadDir(
                    path.to_string_lossy().to_string(),
                ))
            }
        };

        for entry in dir {
            // Possibly continue after failed one file
            if let Ok(entry) = entry {
                let filename = entry.file_name();
                let filename_str = filename.to_string_lossy().to_string();

                if !entry.metadata().map(|md| md.is_file()).unwrap_or(false) {
                    continue;
                }

                let new_folder = match filename_str.split('_').next() {
                    Some(new_folder) => new_folder,
                    None => return Err(HandlerError::InvalidFilename(filename_str.to_string())),
                };
                let new_folder_path = path.join(new_folder);

                let new_path = new_folder_path.join(&filename);
                if let Err(_) = fs::create_dir_all(&new_folder_path) {
                    return Err(HandlerError::FailedToCreateDir(
                        new_folder_path.to_string_lossy().to_string(),
                    ));
                }
                if let Err(_) = fs::rename(path.join(&*filename_str), &new_path) {
                    return Err(HandlerError::FailedToMoveDir(
                        path.join(&*filename_str).to_string_lossy().to_string(),
                        new_path.to_string_lossy().to_string(),
                    ));
                }
            }
        }

        Ok(())
    }

    fn update(&self) -> Result<(), HandlerError> {
        self.move_files_back().unwrap();

        let output = Command::new("gallery-dl")
            .arg(self.update.as_ref().unwrap().as_str())
            .output()
            .unwrap();

        Ok(())
    }
}
