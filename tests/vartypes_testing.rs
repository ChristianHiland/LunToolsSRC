// Libs (NOT USING MINE REMEMBER!)


// Structs
use crate::DateType;


// Tests The 'datetype' struct.
#[test]
fn datetype_test() {
    let test_date: crate::DateType = crate::DateType {month: 11, day: 11, year: 2024};
    println!("Test Date: {}", test_date);
}