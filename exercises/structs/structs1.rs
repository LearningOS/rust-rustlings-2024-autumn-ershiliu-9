// structs1.rs
//
// Address all the TODOs to make the tests pass!
//
// Execute `rustlings hint structs1` or use the `hint` watch subcommand for a
// hint.


// Define the classic struct
struct ColorClassicStruct {
    red: u8,
    green: u8,
    blue: u8,
}

// Define the tuple struct
struct ColorTupleStruct(u8, u8, u8);

// Define the unit-like struct
struct UnitLikeStruct;

// Implement the Debug trait for UnitLikeStruct so it can be printed
impl std::fmt::Debug for UnitLikeStruct {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "UnitLikeStruct")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classic_c_structs() {
        // Instantiate a classic struct!
        let green = ColorClassicStruct {
            red: 0,
            green: 255,
            blue: 0,
        };

        assert_eq!(green.red, 0);
        assert_eq!(green.green, 255);
        assert_eq!(green.blue, 0);
    }

    #[test]
    fn tuple_structs() {
        // Instantiate a tuple struct!
        let green = ColorTupleStruct(0, 255, 0);

        assert_eq!(green.0, 0);
        assert_eq!(green.1, 255);
        assert_eq!(green.2, 0);
    }

    #[test]
    fn unit_structs() {
        // Instantiate a unit-like struct!
        let unit_like_struct = UnitLikeStruct;

        let message = format!("{:?}s are fun!", unit_like_struct);

        assert_eq!(message, "UnitLikeStructs are fun!");
    }
}