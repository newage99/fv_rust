use super::Symbol::Symbol;

pub mod Addition;
pub mod Subtraction;
pub mod SquareRoot;

pub trait Function: Symbol {
    fn min_variables(&self) -> i16 {
        1
    }
    fn max_variables(&self) -> i16 {
        0 // 0 means the function has no limit on the number of variable it can process
    }
    fn parse(&self, variables: Vec<i128>) -> i128;
}
