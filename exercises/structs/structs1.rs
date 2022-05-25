// structs1.rs
// Address all the TODOs to make the tests pass!

const GREEN_COLOR_NAME: &str = "green";
const GREEN_COLOR_HEX: &str = "#00FF00";

struct ColorClassicStruct {
    name: String,
    hex: String,
}

impl ColorClassicStruct {
    fn green() -> ColorClassicStruct {
        ColorClassicStruct{name: String::from(GREEN_COLOR_NAME), hex: String::from(GREEN_COLOR_HEX)}
    }
}

struct ColorTupleStruct(String, String);

impl ColorTupleStruct {
    fn green() -> ColorTupleStruct {
        ColorTupleStruct(String::from(GREEN_COLOR_NAME), String::from(GREEN_COLOR_HEX))
    }
}

#[derive(Debug)]
struct UnitStruct;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classic_c_structs() {
        let green = ColorClassicStruct::green();

        assert_eq!(green.name, "green");
        assert_eq!(green.hex, "#00FF00");
    }

    #[test]
    fn tuple_structs() {
        let green = ColorTupleStruct::green();

        assert_eq!(green.0, "green");
        assert_eq!(green.1, "#00FF00");
    }

    #[test]
    fn unit_structs() {
        let unit_struct = UnitStruct;
        let message = format!("{:?}s are fun!", unit_struct);

        assert_eq!(message, "UnitStructs are fun!");
    }
}
