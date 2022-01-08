use std::fs;

// Verify path exists
pub fn path_exists(path: &str) -> bool {
    fs::metadata(path).is_ok()
}

/*
//Rename files
pub fn rename(original_file:&str, new_name_file:&str) -> std::io::Result<()> {
    fs::rename(original_file, new_name_file)?;
    Ok(())
}
*/