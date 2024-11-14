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
    match read(file, false) {
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


// Tests (Cargo)

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use std::env;
    use super::*;

    // Needed Funcs From The Path Tool
    pub fn get_pwd() -> PathBuf {
        // 1. Gets The Current Working DIR
        let path = env::current_dir().unwrap();
        // 2. Turn it into a String
        path.display().to_string();
        // 3. Retruns the Current Working DIR
        return path;
    }
    /// Gets The Paths of Everything Starting At the Working DIR, then adds with the given path.
    pub fn add_to_PWD(path: String, debug: bool) -> String {
        // 1. Get The Current Working DIR.
        let pwd_path = get_pwd();
        // 2. Convert The &Path into a String.
        let pwd_path = pwd_path.to_str().unwrap().to_string();
        // 3. Add The Current Working DIR with the path that was given by FUNC.
        let added_path = format!("{}{}{}", pwd_path, "\\", path);
        // 4. Returns the Current Working DIR with the 'path' added to the end.
        return added_path;
    }

    #[test]
    fn read_json_test() {
        let jsonfile = add_to_PWD("tests\\data\\test.json".to_string(), false);
        let result = read(jsonfile, false);
        assert!(result.is_ok());
    }

    #[test]
    fn write_json_test() {
        // 1. Setting/Getting the whole path for 'test.json'.
        let jsonfile = add_to_PWD("tests/data/test.json".to_string(), false);
        // 2. Making a json formated String.
        let data = json!({
            "test": "test",
            "new": "new"
        });
        // 3. Write To JSON File.
        write(jsonfile, data, false);
        // 4. Checking for change
        let jsonfile = add_to_PWD("tests/data/test.json".to_string(), false);
        match read(jsonfile, false) {
            Ok(data) => {
                let data_str = data.to_string();
                assert_eq!(data_str, "{\"new\":\"new\",\"test\":\"test\"}");
            },
            Err(e) => println!("Error With Reading JSON File: {}", e),
        }
    }
}