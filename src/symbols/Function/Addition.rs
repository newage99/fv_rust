use super::super::Symbol::Symbol;
use super::super::Function::Function;

pub struct Addition;

impl Symbol for Addition {
    fn symbol(&self) -> &str {
        return "+";
    }
}

impl Function for Addition {
    fn min_variables_needed(&self) -> i16 {
        return 2;
    }
    fn parse(&self, variables: Vec<i128>) -> i128 {
        let mut result: i128 = 0;
        for v in &variables {
            result += v;
        }
        return result;
    }
}