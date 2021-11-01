use super::super::Symbol::Symbol;
use super::super::Function::Function;

pub struct Subtraction;

impl Symbol for Subtraction {
    fn symbol(&self) -> &str {
        return "-";
    }
}

impl Function for Subtraction {
    fn min_variables(&self) -> i16 {
        return 2;
    }
    fn parse(&self, variables: Vec<i128>) -> i128 {
        let mut result: i128 = 0;
        for v in &variables {
            result -= v;
        }
        return result;
    }
}