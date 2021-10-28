use super::super::Symbol::Symbol;
use super::Variable;

pub struct One;

impl Symbol for One {
    fn symbol(&self) -> &str {
        return "1";
    }
}

impl Variable for One {
    fn parse() -> i128 {
        return 1;
    }
}