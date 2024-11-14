
// Libs (NOT USING MINE REMEMBER!)
use serde_json::json;

// Importing Needed Files
use super::super::path::add_path;
use LunTools::json;


#[test]
fn read_json_test() {
    let jsonfile = add_path("tests/data/test.json".to_string(), false);
    let result = json::json_read(jsonfile, false);
    assert!(result.is_ok());
}

#[test]
fn write_json_test() {
    // 1. Setting/Getting the whole path for 'test.json'.
    let jsonfile = add_path("tests/data/test.json".to_string(), false);
    // 2. Making a json formated String.
    let data = json!({
        "test": "test",
        "new": "new"
    });
    // 3. Write To JSON File.
    json::json_write(jsonfile, data, false);
    // 4. Checking for change
    let jsonfile = add_path("tests/data/test.json".to_string(), false);
    match json::json_read(jsonfile, false) {
        Ok(data) => {
            let data_str = data.to_string();
            assert_eq!(data_str, "{\"new\":\"new\",\"test\":\"test\"}");
        },
        Err(e) => println!("Error With Reading JSON File: {}", e),
    }
}