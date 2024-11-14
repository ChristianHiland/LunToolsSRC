// Libs (NOT USING MINE REMEMBER!)
use std::fmt;

// Structs

// Struct: DateStype: month, day
#[derive(Default)]
pub struct DateStype {
    pub month: u8,
    pub day: u8,
}
// Struct: DateType: month, day, year
#[derive(Default)]
pub struct DateType {
    pub month: u8,
    pub day: u8,
    pub year: u16,
}
// Struct: TimeType: hour, min
#[derive(Default)]
pub struct TimeType {
    pub hour: u8,
    pub min: u8
}
// Struct: TimeSecType: hour, min, sec
#[derive(Default)]
pub struct TimeSecType {
    pub hour: u8,
    pub min: u8,
    pub sec: u8,
}
// Struct: DateTime: month, day, year, hour, min, sec
#[derive(Default)]
pub struct DateTimeType {
    pub month: u8,
    pub day: u8,
    pub year: u16,
    pub hour: u8,
    pub min: u8,
    pub sec: u8,
}

// Impl's and Trait's

/// Docstring:
/// Implement Display trait for DateStype to convert to string.
impl fmt::Display for DateStype {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}/{}", self.month, self.day)
    }
}

/// Docstring:
/// Implement Display trait for DateType to convert to string.
impl fmt::Display for DateType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}/{}/{}", self.month, self.day, self.year)           // Customize formatting
    }
}

/// Docstring:
/// Implement display trait for TimeType to convert to string.
impl fmt::Display for TimeType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.hour, self.min)                 //  Formatting Display for Time Type
    }
}

/// Docstring:
/// Implement display trait for TimeSecType to convert to string.
impl fmt::Display for TimeSecType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}:{}", self.hour, self.min, self.sec)    // Formatting Display for Time Sec Type
    }
}

/// Docstring:
/// Implement Display trait for DateTime Type to convert to string.
impl fmt::Display for DateTimeType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}/{}/{} {}:{}:{}", self.month, self.day, self.year, self.hour, self.min, self.sec)
    }
}




// TESTING (cargo)

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// Testing the DateSType Struct, and Type.
    /// 1. Make a Var With all 11 for month, and day.
    /// 2. Convert to String.
    /// 3. Assert that the String is "11/11".
    /// 4. Return ok if it works.
    fn DateStype_Test() {
        let DateStype_Test: DateStype = DateStype {month: 11, day: 11};
        let DateStypeFORMAT_Test: String = DateStype_Test.to_string();
        assert_eq!(DateStypeFORMAT_Test, "11/11");
    }

    #[test]
    /// Testing the DateType Struct, and Type.
    /// 1. Make a Var With all 11 for month, day, and 2024 for year.
    /// 2. Convert to String.
    /// 3. Assert that the String is "11/11/2024".
    /// 4. Return ok if it works.
    fn DateType_Test() {
        let Date_Test: DateType = DateType {month: 11, day: 11, year: 2024};
        let DateFORMAT_Test: String = Date_Test.to_string();
        assert_eq!(DateFORMAT_Test, "11/11/2024");
    }

    #[test]
    /// Testing the DateTimeType Struct, and Type.
    /// 1. Make a Var With all 11 for month, day, hour, min, and sec, and 2024 for year.
    /// 2. Convert to String.
    /// 3. Assert that the String is "11/11/2024 11:11:11".
    /// 4. Return ok if it works.
    fn DateTimeType_Test() {
        let DateTime_Test: DateTimeType = DateTimeType {month: 11, day: 11, year: 2024, hour: 11, min: 11, sec: 11};
        let DateTimeFORMAT_Test: String = DateTime_Test.to_string();
        assert_eq!(DateTimeFORMAT_Test, "11/11/2024 11:11:11");
    }

    #[test]
    /// Testing the TimeType Struct, and Type.
    /// 1. Make a Var With all 11 for hour, and min.
    /// 2. Convert to String.
    /// 3. Assert that the String is "11:11".
    /// 4. Return ok if it works.
    fn TimeType_Test() {
        let Time_Test: TimeType = TimeType {hour: 11, min: 11};
        let TimeFORMAT_Test: String = Time_Test.to_string();
        assert_eq!(TimeFORMAT_Test, "11:11");
    }
    
    #[test]
    /// Testing the TimeSecType Struct, and Type.
    /// 1. Make a Var With all 11 for hour, min, and sec.
    /// 2. Convert to String.
    /// 3. Assert that the String is "11:11:11".
    /// 4. Return ok if it works.
    fn TimeSecType_Test() {
        let TimeSec_Test: TimeSecType = TimeSecType {hour: 11, min: 11, sec: 11};
        let TimeSecFORMAT_Test: String = TimeSec_Test.to_string();
        assert_eq!(TimeSecFORMAT_Test, "11:11:11");
    }
}