use super::super::Symbol::Symbol;
use super::super::Function::Function;

pub struct Addition;

impl Symbol for Addition {
    fn symbol(&self) -> &str {
        return "+";
    }
}

impl Function for Addition {
    fn compute(&self, variables: Vec<i128>) -> i128 {
        if variables.len() > 1 {
            let mut result: i128 = 0;
            for v in &variables {
                result += v;
            }
            return result;
        } else if variables.len() == 1 {
            if variables[0] < 0 {
                return -variables[0];
            }
            return variables[0];
        }
        return 0;
    }
}