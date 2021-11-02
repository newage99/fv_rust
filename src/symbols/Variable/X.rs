use super::super::super::globals::GraphParameters;
use super::super::Symbol::Symbol;
use super::Variable;

pub struct X;

impl Symbol for X {
    fn symbol(&self) -> &str {
        "x"
    }
}

impl Variable for X {
    fn compute(&self, parameters: &GraphParameters) -> i128 {
        parameters.x
    }
}