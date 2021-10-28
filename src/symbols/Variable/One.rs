use super::super::Symbol::Symbol;
use super::Variable;

struct One;

impl Symbol for One {
    fn symbol() -> String {
        return String::from("1");
    }
}

impl Variable for One {
    fn parse() -> i128 {
        return 1;
    }
}