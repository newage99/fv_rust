use super::super::Symbol::Symbol;
use super::super::Function::Function;

pub struct Modulus;

impl Symbol for Modulus {
    fn symbol(&self) -> &str {
        return "%";
    }
}

impl Function for Modulus {
    fn min_variables(&self) -> i128 {
        return 2;
    }
    fn compute(&self, variables: &Vec<i128>) -> i128 {
        let mut first: bool = true;
        let mut result: i128 = 0;
        for v in variables {
            if *v == 0 {
                return 0;
            }
            if first {
                result = *v;
                first = false;
            } else {
                result = result % *v;
            }
        }
        result
    }
}