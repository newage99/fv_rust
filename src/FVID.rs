use super::globals::GlobalVariables;
use super::globals::GraphParameters;
use super::symbols::Symbol::Symbol;
use super::symbols::Function::Function;
use super::Graph::Graph;

pub struct FVID {
    id: Vec<&'static dyn Symbol>
}

fn min_variables_not_satisfied_error(symbol_str: &str, min_variables: i128, position: i128) -> String {
    let mut return_str: String = String::from("Function '");
    return_str.push_str(symbol_str);
    return_str.push_str("' at position ");
    return_str.push_str(&position.to_string());
    return_str.push_str(" cannot process less than ");
    return_str.push_str(&min_variables.to_string().to_owned());
    return_str.push_str(" variables.");
    return return_str;
}

fn max_variables_not_satisfied_error(symbol_str: &str, max_variables: i128, position: i128) -> String {
    let mut return_str: String = String::from("Function '");
    return_str.push_str(symbol_str);
    return_str.push_str("' at position ");
    return_str.push_str(&position.to_string());
    return_str.push_str(" cannot process more than ");
    return_str.push_str(&max_variables.to_string().to_owned());
    return_str.push_str(" variables.");
    return return_str;
}

impl FVID {

    pub fn check_symbols_list(symbols_list: &Vec<&dyn Symbol>, global_variables: &GlobalVariables) -> (String, i128) {
        let mut str_list: Vec<&str> = Vec::new();
        for symbol in symbols_list {
            str_list.push(symbol.symbol());
        }
        return FVID::check_str_list(str_list, global_variables);
    }

    pub fn check_str(fvid: &str, global_variables: &GlobalVariables) -> (String, i128) {
        let str_list: Vec<&str> = fvid.split(" ").collect();
        return FVID::check_str_list(str_list, global_variables);
    }

    pub fn check_str_list(str_list: Vec<&str>, global_variables: &GlobalVariables) -> (String, i128) {

        let mut first: bool = true;
        let mut number_of_variables_processesed: i128 = 0;
        let mut last_function_option: Option<&dyn Function> = None;
        let mut last_function_position: i128 = 0;
        let mut position: i128 = 0;

        for fvunit in str_list {
            if !global_variables.symbols_map.contains_key(fvunit) {
                let mut return_str: String = String::from("Not recognized symbol '");
                return_str.push_str(fvunit);
                return_str.push_str("'.");
                return (return_str, position);
            }
            let fvunit_is_a_function = global_variables.functions_map.contains_key(fvunit);
            if first {
                if !fvunit_is_a_function {
                    let mut return_str: String = String::from("FVID must start with a function. '");
                    return_str.push_str(fvunit);
                    return_str.push_str("' is not a function.");
                    return (return_str, position);
                }
                first = false;
                last_function_option = Some(global_variables.functions_map[fvunit]);
            } else {
                match last_function_option {
                    Some(last_function) => {
                        let max_variables: i128 = last_function.max_variables();
                        if max_variables > 0 && number_of_variables_processesed > max_variables {
                            let error = max_variables_not_satisfied_error(last_function.symbol(), max_variables, last_function_position);
                            return (error, position + number_of_variables_processesed);
                        }
                        if fvunit_is_a_function {
                            let min_variables_needed: i128 = last_function.min_variables();
                            if number_of_variables_processesed + 1 < min_variables_needed {
                                let error = min_variables_not_satisfied_error(last_function.symbol(), min_variables_needed, last_function_position);
                                return (error, position + number_of_variables_processesed);
                            }
                            last_function_option = Some(global_variables.functions_map[fvunit]);
                            last_function_position = position;
                            number_of_variables_processesed = 0;
                        } else {
                            number_of_variables_processesed += 1;
                        }
                    }
                    None => {}
                }
            }
            position += 1;
        }
        match last_function_option {
            Some(last_function) => {
                let max_variables: i128 = last_function.max_variables();
                if max_variables > 0 && number_of_variables_processesed > max_variables {
                    let error = max_variables_not_satisfied_error(last_function.symbol(), max_variables, last_function_position);
                    return (error, position + number_of_variables_processesed);
                }
                let min_variables_needed: i128 = last_function.min_variables();
                if number_of_variables_processesed < min_variables_needed {
                    let error = min_variables_not_satisfied_error(last_function.symbol(), min_variables_needed, last_function_position);
                    return (error, position + number_of_variables_processesed);
                }
            }
            None => {}
        }
        return (String::new(), -1);
    }

    pub fn create(fvid: &str, global_variables: &GlobalVariables) -> FVID {
        let split: Vec<&str> = fvid.split(" ").collect();
        let mut fvid_vec: Vec<&dyn Symbol> = Vec::new();
        for fvunit in split {
            fvid_vec.push(global_variables.symbols_map[fvunit]);
        }
        let fvid: FVID = FVID {
            id: fvid_vec
        };
        fvid
    }

    fn get_next_symbol(prev_symbol: &'static dyn Symbol, global_variables: &GlobalVariables) -> &'static dyn Symbol {
        let mut get_next_symbol_bool: bool = false;
        let mut next_symbol: &dyn Symbol = prev_symbol;
        for symbol in &global_variables.symbols_list {
            if get_next_symbol_bool {
                next_symbol = *symbol;
                break;
            }
            if prev_symbol.symbol() == symbol.symbol() {
                get_next_symbol_bool = true;
            }
        }
        next_symbol
    }

    pub fn create_all_for_number_of_symbols(number_of_symbols: i128, global_variables: &GlobalVariables) -> Vec<FVID> {

        println!("create_all_for_number_of_symbols");

        let mut fvids: Vec<FVID> = Vec::new();

        if number_of_symbols > 1 {

            /*let mut new_fvid: &FVID = &FVID {
                id: vec![]
            };*/
            let mut fvid_symbols_list: Vec<&dyn Symbol> = Vec::new();
            for i in 0..number_of_symbols {
                //new_fvid.id.push(global_variables.symbols_list[0]);
                fvid_symbols_list.push(global_variables.symbols_list[0]);
            }
            let mut not_finished = true;
            while not_finished {
                print!("FVID: ");
                //for fvid_symbol in new_fvid.id.iter() {
                for fvid_symbol in &fvid_symbols_list {
                    print!("{} ", fvid_symbol.symbol());
                }
                println!("");
                let check_response: (String, i128) = FVID::check_symbols_list(&fvid_symbols_list, global_variables);
                //let check_response: (String, i128) = (String::new(), 0);
                //let pos_to_change: i128 = number_of_symbols - 1;
                if check_response.0 == "" {  
                    //fvid_symbols_list = fvid_symbols_list.to_vec();
                    let new_fvid: FVID = FVID {
                        id: fvid_symbols_list.to_vec()
                    };
                    fvids.push(new_fvid);
                }/* else if check_response.1 >= 0 {
                    pos_to_change = check_response.1;
                }*/
                /*for i in (0..number_of_symbols).rev() {
                }*/
                /*new_fvid = &FVID {
                    id: new_fvid.id.to_vec()
                };*/
                for i in (0..fvid_symbols_list.len()).rev() {
                //for symbol in fvid_symbols_list.iter().rev() {
                    let symbol: &dyn Symbol = fvid_symbols_list[i];
                    if symbol.symbol() != global_variables.symbols_list[global_variables.symbols_list.len() - 1].symbol() {
                        //*symbol = FVID::get_next_symbol(*symbol, global_variables);
                        fvid_symbols_list[i] = FVID::get_next_symbol(symbol, global_variables);
                        break;
                    } else if i > 0 {
                        //*symbol = global_variables.symbols_list[0];
                        fvid_symbols_list[i] = global_variables.symbols_list[0];
                    } else {
                        not_finished = false;
                    }
                }
            }
        }

        fvids
    }

    fn compute_for_xy(&self, number_of_nodes: i128, x: i128, y: i128, global_variables: &GlobalVariables) -> i128 {
        let mut variables_to_process: Vec<i128> = Vec::new();
        let graph_parameters: GraphParameters = GraphParameters {
            x: x,
            y: y,
            number_of_nodes: number_of_nodes
        };
        for symbol in self.id.iter().rev() {
            let symbol_str: &str = symbol.symbol();
            if global_variables.functions_map.contains_key(symbol.symbol()) {
                let result: i128 = global_variables.functions_map[symbol_str].compute(variables_to_process);
                variables_to_process = vec![result];
            } else {
                let result: i128 = global_variables.variables_map[symbol_str].compute(&graph_parameters);
                variables_to_process.insert(0, result);
            }
        }
        return variables_to_process[0];
    }

    pub fn compute(&self, number_of_nodes: i128, global_variables: &GlobalVariables) -> Graph {
        
        let mut matrix: Vec<Vec<i128>> = Vec::new();

        for x in 0..(number_of_nodes - 1) {
            let mut new_vec = Vec::new();
            for y in (x + 1)..number_of_nodes {
                new_vec.push(self.compute_for_xy(number_of_nodes, x, y, global_variables));
            }
            matrix.push(new_vec);
        }

        let graph: Graph = Graph {
            adjacency_matrix: matrix
        };

        graph
    }
}