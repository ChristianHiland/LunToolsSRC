// Libs (NOT USING MINE REMEMBER!)
use std::fmt;                                       // Importing Fmt from std.

// Structs

// Struct: DB_Lite_people: name, age
pub struct DB_Lite_people {
    pub name: String,
    pub age: u8,
}
// Struct: DB_Base_people: name, age, date of brith, last login.
pub struct DB_Base_people {
    pub name: String,
    pub age: u8,
    pub dob: (u8, u8, u16),
    pub last_login: (u8, u8, u16, u8, u8, u8),
}
// Struct: DB_CUS_people: name, age, date of brith, last login.
pub struct DB_CUS_people {
    pub name: String,
    pub age: u8,
    pub dob: (u8, u8, u16),
    pub last_login: (u8, u8, u16, u8, u8, u8),
    pub pfp: String,
}

// Impl's

/// Docstring:
/// Implement Display trait for DB_Lite_people to convert to string.
impl fmt::Display for DB_Lite_people {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Name: {} Age: {}", self.name, self.age)
    }
}

/// Docstring:
/// Implement Display trait for DB_Base_people to convert to string.
impl fmt::Display for DB_Base_people {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Name: {} Age: {} Date of Birth: {} Last Login: {}", self.name, self.age, self.dob, self.last_login)
    }
}

/// Docstring:
/// Implement Display trait for DB_CUS_people to convert to string.
impl fmt::Display for DB_CUS_people {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Name: {} Age: {} Date of Birth: {} Last Login: {} PFP: {}", self.name, self.age, self.dob, self.last_login, self.pfp)
    }
}


// TESTING (cargo)

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// Testing the DB_Base_people Struct, and Type.
    /// 1. Make a Var With {name: "leelunbin".to_string(), age: 15, dob: (10, 13, 2009), (11, 11, 2024, 7, 31, 50)}
    /// 2. Convert to String.
    /// 3. Assert that the String is "Name: leelunbin Age: 15 Date of Birth: 10, 13, 2009 Last Login: 11, 11, 2024, 7, 31, 50".
    /// 4. Return ok if it works.
    fn DB_Base_people_Test() {
        let DB_Base_people_STest: DB_Base_people = DB_Base_people {name: "leelunbin".to_string(), age: 15, dob: (10, 13, 2009), (11, 11, 2024, 7, 31, 50)};
        let DB_Base_peopleFORMAT_Test: String = DB_Base_people_STest.to_string();
        assert_eq!(DB_Base_peopleFORMAT_Test, "Name: leelunbin Age: 15 Date of Birth: 10, 13, 2009 Last Login: 11, 11, 2024, 7, 31, 50");
    }

    #[test]
    /// Testing the DateSType Struct, and Type.
    /// 1. Make a Var With {name: "leelunbin".to_string(), age: 15, dob: (10, 13, 2009), (11, 11, 2024, 7, 31, 50)}
    /// 2. Convert to String.
    /// 3. Assert that the String is "Name: leelunbin Age: 15 Date of Birth: 10, 13, 2009 Last Login: 11, 11, 2024, 7, 31, 50".
    /// 4. Return ok if it works.
    fn DB_Lite_people_Test() {
        let DB_Lite_people_STest: DB_Lite_people = DB_Lite_people {name: "leelunbin".to_string(), age: 15};
        let DB_Lite_peopleFORMAT_Test: String = DB_Lite_people_STest.to_string();
        assert_eq!(DB_Lite_peopleFORMAT_Test, "Name: leelunbin Age: 15");
    }

    #[test]
    /// Testing the DB_CUS_people Struct, and Type.
    /// 1. Make a Var With {name: "leelunbin".to_string(), age: 15, dob: (10, 13, 2009), (11, 11, 2024, 7, 31, 50)}
    /// 2. Convert to String.
    /// 3. Assert that the String is "Name: leelunbin Age: 15 Date of Birth: 10, 13, 2009 Last Login: 11, 11, 2024, 7, 31, 50".
    /// 4. Return ok if it works.
    fn DB_CUS_people_Test() {
        let DB_CUS_people_STest: DB_CUS_people = DB_CUS_people {name: "leelunbin".to_string(), age: 15, dob: (10, 13, 2009), (11, 11, 2024, 7, 31, 50), pfp: "leelunbin.png".to_string()};
        let DB_CUS_peopleFORMAT_Test: String = DB_CUS_people_STest.to_string();
        assert_eq!(DB_CUS_peopleFORMAT_Test, "Name: leelunbin Age: 15 Date of Birth: 10, 13, 2009 Last Login: 11, 11, 2024, 7, 31, 50, PFP: leelunbin.png");
    }
}