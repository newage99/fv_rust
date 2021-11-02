use super::super::Symbol::Symbol;
use super::super::Function::Function;
use num_integer::Roots;

pub struct SquareRoot;

impl Symbol for SquareRoot {
    fn symbol(&self) -> &str {
        return "√";
    }
}

impl Function for SquareRoot {
    fn max_variables(&self) -> i128 {
        return 1;
    }
    fn compute(&self, variables: Vec<i128>) -> i128 {
        for var in &variables {
            if var <= &0 {
                return 0;
            }
            return var.nth_root(2);
        }
        return 0;
    }
}