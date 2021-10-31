use super::super::globals::GlobalVariables;

pub trait Command {
    fn text() -> String;
    fn run(parameters: Vec<&str>, global_variables: GlobalVariables);
    fn help() -> String;
}