use std::{
    fs, // Import standard files and folders manipulation
    process::Command, // Invoke System Commands
};


pub fn path_exists(path: &str) -> bool {
    fs::metadata(path).is_ok()
}

pub fn systemcommand_asuser(package: &str, command: &str, err: &str) {
    Command::new(package).args(command.split_ascii_whitespace()).status().expect(err);
}

pub fn systemcommand_asroot(command: &str, err: &str) {
    Command::new("sudo").args(command.split_ascii_whitespace()).status().expect(err);
}