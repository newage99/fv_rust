extern crate num_integer;
mod symbols;
mod commands;
mod FVID;
mod Graph;
mod Topology;
mod globals;
use globals::GlobalVariables;
use symbols::Symbol::Symbol;
use symbols::Function::Function;
use symbols::Variable::Variable;
use symbols::Function::Addition::Addition;
use symbols::Function::Division::Division;
use symbols::Function::Modulus::Modulus;
use symbols::Function::Multiplication::Multiplication;
use symbols::Function::SquareRoot::SquareRoot;
use symbols::Function::Subtraction::Subtraction;
use symbols::Variable::One::One;
use symbols::Variable::X::X;
use symbols::Variable::Y::Y;
use symbols::Variable::NumberOfNodes::NumberOfNodes;
use std::collections::HashMap;
use commands::Command::Command;
use commands::RunCommand::RunCommand;

fn main() {

    let symbols: Vec<&dyn Symbol> =     vec![&Addition, &Division, &Modulus, &Multiplication, &SquareRoot, &Subtraction, &X, &Y, &NumberOfNodes, &One];
    let functions: Vec<&dyn Function> = vec![&Addition, &Division, &Modulus, &Multiplication, &SquareRoot, &Subtraction];
    let variables: Vec<&dyn Variable> = vec![&X, &Y, &NumberOfNodes, &One];
    
    let mut global_variables: GlobalVariables = GlobalVariables {
        symbols_list: symbols,
        functions_list: functions,
        variables_list: variables,
        symbols_map: HashMap::new(),
        functions_map: HashMap::new(),
        variables_map: HashMap::new()
    };

    for symbol in &global_variables.functions_list {
        global_variables.functions_map.insert(<dyn Function as Symbol>::symbol(*symbol), *symbol);
    }
    for symbol in &global_variables.variables_list {
        global_variables.variables_map.insert(<dyn Variable as Symbol>::symbol(*symbol), *symbol);
    }
    for symbol in &global_variables.symbols_list {
        global_variables.symbols_map.insert(symbol.symbol(), *symbol);
    }

    RunCommand::run(vec!["id=* √ - y x"], global_variables);
}