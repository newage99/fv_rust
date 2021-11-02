use super::super::super::globals::GraphParameters;
use super::super::Symbol::Symbol;
use super::Variable;

pub struct Y;

impl Symbol for Y {
    fn symbol(&self) -> &str {
        "y"
    }
}

impl Variable for Y {
    fn compute(&self, parameters: &GraphParameters) -> i128 {
        parameters.y
    }
}