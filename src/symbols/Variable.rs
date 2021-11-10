use super::super::globals::GraphParameters;
use super::Symbol::Symbol;

pub mod One;
pub mod Two;
pub mod X;
pub mod Y;
pub mod NumberOfNodes;

pub trait Variable: Symbol {
    fn compute(&self, parameters: &GraphParameters) -> i128;
}