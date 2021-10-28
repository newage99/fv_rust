use super::super::Symbol::Symbol;
use super::Function;

pub struct Add;

impl Symbol for Add {
    fn symbol() -> String {
        return String::from("+");
    }
}

impl Function for Add {
    fn accepts_multiple_variables() -> bool {
        return true;
    }
    fn parse(variables: Vec<i128>) -> i128 {
        let mut result: i128 = 0;
        for v in &variables {
            result += v;
        }
        return result;
    }
}