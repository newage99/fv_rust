use super::globals::GlobalVariables;
use super::globals::GraphParameters;
use super::symbols::Symbol::Symbol;
use super::symbols::Function::Function;
use super::Graph::Graph;

pub struct FVID {
    id: Vec<&'static dyn Symbol>
}

fn min_variables_not_satisfied_error(symbol_str: &str, min_variables: i16, position: i32) -> String {
    let mut return_str: String = String::from("Function '");
    return_str.push_str(symbol_str);
    return_str.push_str("' at position ");
    return_str.push_str(&position.to_string());
    return_str.push_str(" cannot process less than ");
    return_str.push_str(&min_variables.to_string().to_owned());
    return_str.push_str(" variables.");
    return return_str;
}

fn max_variables_not_satisfied_error(symbol_str: &str, max_variables: i16, position: i32) -> String {
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
    pub fn check(fvid: &str, global_variables: &GlobalVariables) -> String {

        let split: Vec<&str> = fvid.split(" ").collect();
        let mut first: bool = true;
        let mut number_of_variables_processesed: i16 = 0;
        let mut last_function_option: Option<&dyn Function> = None;
        let mut last_function_position: i32 = 0;
        let mut position: i32 = 0;

        for fvunit in split {
            if !global_variables.symbols_map.contains_key(fvunit) {
                let mut return_str: String = String::from("Not recognized symbol '");
                return_str.push_str(fvunit);
                return_str.push_str("'.");
                return return_str;
            }
            let fvunit_is_a_function = global_variables.functions_map.contains_key(fvunit);
            if first {
                if !fvunit_is_a_function {
                    let mut return_str: String = String::from("FVID must start with a function. '");
                    return_str.push_str(fvunit);
                    return_str.push_str("' is not a function.");
                    return return_str;
                }
                first = false;
                last_function_option = Some(global_variables.functions_map[fvunit]);
            } else {
                match last_function_option {
                    Some(last_function) => {
                        let max_variables: i16 = last_function.max_variables();
                        if max_variables > 0 && number_of_variables_processesed > max_variables {
                            return max_variables_not_satisfied_error(last_function.symbol(), max_variables, last_function_position);
                        }
                        if fvunit_is_a_function {
                            let min_variables_needed: i16 = last_function.min_variables();
                            if number_of_variables_processesed + 1 < min_variables_needed {
                                return min_variables_not_satisfied_error(last_function.symbol(), min_variables_needed, last_function_position);
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
                let max_variables: i16 = last_function.max_variables();
                if max_variables > 0 && number_of_variables_processesed > max_variables {
                    return max_variables_not_satisfied_error(last_function.symbol(), max_variables, last_function_position);
                }
                let min_variables_needed: i16 = last_function.min_variables();
                if number_of_variables_processesed < min_variables_needed {
                    return min_variables_not_satisfied_error(last_function.symbol(), min_variables_needed, last_function_position);
                }
            }
            None => {}
        }
        return String::new();
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

        for x in 0..number_of_nodes {
            let mut new_vec = Vec::new();
            for y in x..number_of_nodes {
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