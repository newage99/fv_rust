use super::super::super::globals::GraphParameters;
use super::super::Symbol::Symbol;
use super::Variable;

pub struct NumberOfNodes;

impl Symbol for NumberOfNodes {
    fn symbol(&self) -> &str {
        "n"
    }
}

impl Variable for NumberOfNodes {
    fn compute(&self, parameters: &GraphParameters) -> i128 {
        parameters.number_of_nodes
    }
}