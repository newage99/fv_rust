use super::Symbol::Symbol;

pub mod One;

pub trait Variable: Symbol {
    fn parse(&self) -> i128;
    // Implementations of Variable must also implement Symbol trait functions
    fn symbol(&self) -> &str;
}