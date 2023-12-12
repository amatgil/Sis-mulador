pub mod execute;
pub mod parsing;
pub mod spec;

pub use std::num::ParseIntError;
pub use spec::*;
pub use parsing::*;
pub use execute::*;

pub fn norm_n(input: &str) -> Result<usize, ParseIntError> {
    if input.len() <= 2 || &input[..2] != "0x" { input.parse() } // Is dec here
    else {  usize::from_str_radix(&input[2..], 16) }
}


#[test]
fn test_norm() {
    use std::collections::HashMap;

    let pairs = HashMap::from([
        ("0x0000", 0), ("0x0001", 1), ("0x0010", 16),
        ("0x0015", 21), ("0x0020", 16*2), ("0", 0),
        ("1", 1), ("10", 10), ("15", 15), ("20", 20),
    ]);
    for (k, v) in pairs {
        dbg!(k, v);
        assert_eq!(norm_n(k), Ok(v));
    }
}

