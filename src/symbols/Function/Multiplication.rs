use super::super::Symbol::Symbol;
use super::super::Function::Function;

pub struct Multiplication;

impl Symbol for Multiplication {
    fn symbol(&self) -> &str {
        return "*";
    }
}

impl Function for Multiplication {
    fn compute(&self, variables: &Vec<i128>) -> i128 {
        if variables.len() > 1 {
            let mut result: i128 = 1;
            for v in variables {
                result *= *v;
            }
            return result;
        } else if variables.len() == 1 {
            let response = variables[0].checked_pow(2);
            let mut result: i128 = 1;
            match response {
                Some(number) => {
                    result = number;
                },
                None => {}
            }
            return result;
        }
        return 0;
    }
}