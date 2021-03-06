use super::super::super::globals::GraphParameters;
use super::super::Symbol::Symbol;
use super::Variable;

pub struct One;

impl Symbol for One {
    fn symbol(&self) -> &str {
        "1"
    }
}

impl Variable for One {
    fn compute(&self, parameters: &GraphParameters) -> i128 {
        1
    }
}