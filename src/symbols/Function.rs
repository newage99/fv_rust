use super::Symbol::Symbol;

pub mod Addition;
pub mod Subtraction;

pub trait Function: Symbol {
    fn accepts_multiple_variables(&self) -> bool;
    fn parse(&self, variables: Vec<i128>) -> i128;
}
