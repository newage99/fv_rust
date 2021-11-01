use super::super::Symbol::Symbol;
use super::super::Function::Function;

pub struct SquareRoot;

impl Symbol for SquareRoot {
    fn symbol(&self) -> &str {
        return "√";
    }
}

impl Function for SquareRoot {
    fn max_variables(&self) -> i16 {
        return 1;
    }
    fn parse(&self, variables: Vec<i128>) -> i128 {
        for var in variables {
            if var < 0 {
                return 0;
            }
        }
        return 0;
    }
}