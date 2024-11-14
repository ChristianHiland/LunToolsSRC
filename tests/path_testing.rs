
// Libs (NOT USING MINE REMEMBER!)
use crate::path;

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