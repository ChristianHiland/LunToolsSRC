//! Lunbin Path Tool
//! This is a modules/script that contains tools to find the PWD, Find a file.

// Libs (NOT USING MINE REMEMBER!)
use std::path::PathBuf;
use std::env;

/// Gets The PWD Path of Everything Starting At The Working DIR.
pub fn get_pwd() -> PathBuf {
    // 1. Gets The Current Working DIR
    let path = env::current_dir().unwrap();
    // 2. Turn it into a String
    path.display().to_string();
    // 3. Retruns the Current Working DIR
    return path;
}

/// Gets The Exe Path of The Current Program.
pub fn get_exe() -> PathBuf {
    // 1. Get The Executeable Path
    let exe_path = env::current_exe().unwrap();
    // 2. Turn it into a String
    exe_path.display().to_string();
    // 3. Returns the Current Program Exe Path.
    return exe_path;
}

/// Gets The Paths of Everything Starting At the Working DIR, then adds with the given path.
pub fn add_to_PWD(path: String, debug: bool) -> String {
    // 1. Get The Current Working DIR.
    let pwd_path = get_pwd();
    // 2. Convert The &Path into a String.
    let pwd_path = pwd_path.to_str().unwrap().to_string();
    // 3. Add The Current Working DIR with the path that was given by FUNC.
    let added_path = format!("{}{}{}", pwd_path, "\\src\\", path);
    // 4. Returns the Current Working DIR with the 'path' added to the end.
    return added_path;
}


// Tests (Cargo)

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn path_test() {
        println!("Path Test");
    }
    // Tests The 'get_pwd' func.
    #[test]
    fn get_pwd_test() {
        path::get_pwd();
    }
    // Tests The 'get_exe' func.
    #[test]
    fn get_exe_tet() {
        path::get_exe();
    }
    // Tests The 'add_path' func.
    #[test]
    fn add_path_test() {
        let result = path::add_path("tests/data/test.json".to_string(), false);
        assert_eq!(result, "C:\\Users\\hilan\\Documents\\Code\\Rust\\LunTools\\LunToolsSRC\\tests\\data\\test.json".to_string());
    }
}
