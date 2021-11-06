use super::super::Symbol::Symbol;
use super::super::Function::Function;
use num_integer::Integer;

pub struct Division;

impl Symbol for Division {
    fn symbol(&self) -> &str {
        return "/";
    }
}

impl Function for Division {
    fn min_variables(&self) -> i128 {
        2
    }
    fn compute(&self, variables: &Vec<i128>) -> i128 {
        let mut result: i128 = 0;
        let mut first: bool = true;
        for v in variables {
            if *v == 0 {
                return 0;
            }
            if first {
                result = *v;
                first = false;
            } else {
                result = result.div_floor(v);
            }
        }
        return result;
    }
}