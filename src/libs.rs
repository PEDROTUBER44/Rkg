use std::{
    fs
};

// Verify path exists
pub fn path_exists(path: &str) -> bool {
    fs::metadata(path).is_ok()
}