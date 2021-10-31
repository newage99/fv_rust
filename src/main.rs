mod symbols;
mod commands;
mod FVID;
mod globals;
use globals::GlobalVariables;
use symbols::Symbol::Symbol;
use symbols::Function::Function;
use symbols::Variable::Variable;
use symbols::Function::Addition::Addition;
use symbols::Function::Subtraction::Subtraction;
use symbols::Variable::One::One;
use std::collections::HashMap;
use commands::Command::Command;
use commands::RunCommand::RunCommand;

fn main() {

    let symbols: Vec<&dyn Symbol> =     vec![&Addition, &Subtraction, &One];
    let functions: Vec<&dyn Function> = vec![&Addition, &Subtraction];
    let variables: Vec<&dyn Variable> = vec![&One];
    
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

    RunCommand::run(vec!["id=1 + 1"], global_variables);
}