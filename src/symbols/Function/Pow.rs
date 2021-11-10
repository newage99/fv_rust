use super::super::Symbol::Symbol;
use super::super::Function::Function;
use num_integer::Roots;

pub struct Pow;

impl Symbol for Pow {
    fn symbol(&self) -> &str {
        return "^";
    }
}

impl Function for Pow {
    fn compute(&self, variables: &Vec<i128>) -> i128 {
        if variables.len() > 1 {
            let mut result: i128 = 0;
            let mut first: bool = true;
            let mut response;
            for v in variables {
                if first {
                    result = *v;
                    first = false;
                } else {
                    response = result.checked_pow(*v as u32);
                    match response {
                        Some(number) => {
                            result = number;
                        },
                        None => {
                            result = 1;
                        }
                    }
                }
            }
            return result;
        } else if variables.len() == 1 {
            let response = variables[0].checked_pow(variables[0] as u32);
            let mut result: i128 = 1;
            match response {
                Some(number) => {
                    result = number;
                },
                None => {}
            }
            return result;
        }
        0
    }
}