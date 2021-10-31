use super::super::Symbol::Symbol;
use super::Variable;

pub struct One;

fn get_symbol() -> &'static str {
    return "1";
}

impl Symbol for One {
    fn symbol(&self) -> &str {
        return get_symbol();
    }
}

impl Variable for One {
    fn parse(&self) -> i128 {
        return 1;
    }
    fn symbol(&self) -> &str {
        return get_symbol();
    }
}