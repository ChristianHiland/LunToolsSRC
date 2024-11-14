//! Lunbin JSON Tool
//! This is a modules/script that contains tools to read, write, maybe some more.

// Libs (NOT USING MINE REMEMBER!)
use serde_json::to_writer_pretty;
use serde_json::json;
use std::fs::File;
use std::fs;

// Funcs

/// Docstring: JSON Read By String Tool
/// Input: JSON_BASED_STRING
/// OUTPUT: Rust Arrary
pub fn read(file: String, debug: bool) -> Result<serde_json::Value, std::io::Error> {
    // 1. Read the JSON file, put it into as a String.
    let json_string = fs::read_to_string(file)?;
    // 2. Deserialize the JSON string into Rust Data structure.
    let data: serde_json::Value = serde_json::from_str(&json_string)?;
    // 3. Return the "serde_json::Value" value that was read from the file given.
    Ok(data)
}

/// Docstring: JSON Write By String Tool
/// Input: JSON_BASED_STRING
/// Output: result
pub fn write(file: String, data: serde_json::Value, debug:bool) -> i8 {
    let mut file_writing = File::create(file).expect("Unable to create file!");
    to_writer_pretty(file_writing, &data).expect("Unable to write data!");

    // Returning '0' meaning exited with good.
    return 0;
}

/// Docstring: JSON Verify
/// Input: file to check, data to check with.
/// Output: bool
pub fn verify(file: String, data: serde_json::Value) -> bool {
    // 1. Verify
    match json_read(file, false) {
        Ok(file_data) => {
            if file_data == data {
                return true;
            }
            else {
                return false;
            }
        },
        Err(e) => {
            println!("Error With Reading JSON File: {}", e);
            return false;
        }
    }
}