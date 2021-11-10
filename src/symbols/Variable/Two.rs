use super::super::super::globals::GraphParameters;
use super::super::Symbol::Symbol;
use super::Variable;

pub struct Two;

impl Symbol for Two {
    fn symbol(&self) -> &str {
        "2"
    }
}

impl Variable for Two {
    fn compute(&self, parameters: &GraphParameters) -> i128 {
        2
    }
}