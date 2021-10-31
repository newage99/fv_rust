use super::Symbol::Symbol;

pub mod Addition;
pub mod Subtraction;

pub trait Function: Symbol {
    fn min_variables_needed(&self) -> i16;
    fn parse(&self, variables: Vec<i128>) -> i128;
}
