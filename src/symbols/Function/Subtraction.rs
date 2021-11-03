use super::super::Symbol::Symbol;
use super::super::Function::Function;

pub struct Subtraction;

impl Symbol for Subtraction {
    fn symbol(&self) -> &str {
        return "-";
    }
}

impl Function for Subtraction {
    fn compute(&self, variables: Vec<i128>) -> i128 {
        if variables.len() > 1 {
            let mut result: i128 = 0;
            let mut first: bool = true;
            for v in &variables {
                if first {
                    result = *v;
                    first = false;
                } else {
                    result -= v;
                }
                //print!("{} ", result);
            }
            //println!("");
            return result;
        } else if variables.len() == 1 {
            return -variables[0];
        }
        return 0;
    }
}