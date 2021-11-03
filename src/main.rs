extern crate num_integer;
mod symbols;
mod commands;
mod FVID;
mod Graph;
mod globals;
mod print;
use globals::GlobalVariables;
use symbols::Symbol::Symbol;
use symbols::Function::Function;
use symbols::Variable::Variable;
use symbols::Function::Addition::Addition;
use symbols::Function::Modulus::Modulus;
use symbols::Function::SquareRoot::SquareRoot;
use symbols::Function::Subtraction::Subtraction;
use symbols::Variable::One::One;
use symbols::Variable::X::X;
use symbols::Variable::Y::Y;
use symbols::Variable::NumberOfNodes::NumberOfNodes;
use std::collections::HashMap;
use commands::Command::Command;
use commands::RunCommand::RunCommand;
use print::make_even;
use print::output;

fn main() {

    let symbols: Vec<&dyn Symbol> =     vec![&Addition, &Modulus, &SquareRoot, &Subtraction, &X, &Y, &NumberOfNodes, &One];
    let functions: Vec<&dyn Function> = vec![&Addition, &Modulus, &SquareRoot, &Subtraction];
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

    /*let mut char_index = 0;
    let three_bytes = vec![224, 239, 64, 191, 64, 191];
    for i in three_bytes[0]..three_bytes[1] {
        for j in three_bytes[2]..three_bytes[3] {
            for k in three_bytes[4]..three_bytes[5] {
                let mut first = format!("{:X}", i);
                let mut second = format!("{:X}", j);
                let mut third = format!("{:X}", k);

                first = make_even(first);
                second = make_even(second);
                third = make_even(third);

                char_index = output(
                    first.to_string() + &second.to_string() + &third.to_string(),
                    char_index,
                );
            }
        }
    }*/

    RunCommand::run(vec!["id=+ 1 1"], global_variables);
}