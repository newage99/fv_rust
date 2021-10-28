pub mod Addition;

pub trait Function {
    fn accepts_multiple_variables() -> bool;
    fn parse(variables: Vec<i128>) -> i128;
}