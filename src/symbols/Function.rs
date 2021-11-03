use super::Symbol::Symbol;

pub mod Addition;
pub mod Modulus;
pub mod SquareRoot;
pub mod Subtraction;

pub trait Function: Symbol {
    fn min_variables(&self) -> i128 {
        1
    }
    fn max_variables(&self) -> i128 {
        0 // 0 means the function has no limit on the number of variable it can process
    }
    fn compute(&self, variables: Vec<i128>) -> i128;
}