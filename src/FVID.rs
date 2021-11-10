use chrono::Utc;

use super::globals::GlobalVariables;
use super::globals::GraphParameters;
use super::symbols::Symbol::Symbol;
use super::symbols::Function::Function;
use super::Graph::Graph;

pub struct FVID {
    pub id: Vec<&'static dyn Symbol>
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

    pub fn to_string(&self) -> String {
        let mut response: String = String::new();
        for symbol in &self.id {
            response.push_str(symbol.symbol());
            response.push(' ');
        }
        response.remove(response.len() - 1);
        response
    }

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

    pub fn get_next_symbol(prev_symbol: &'static dyn Symbol, global_variables: &GlobalVariables) -> &'static dyn Symbol {
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

        let mut fvids: Vec<FVID> = Vec::new();

        if number_of_symbols > 1 {

            let mut fvid_symbols_list: Vec<&dyn Symbol> = Vec::new();
            for i in 0..number_of_symbols {
                fvid_symbols_list.push(global_variables.symbols_list[0]);
            }
            let mut not_finished = true;

            while not_finished {

                let check_response: (String, i128) = FVID::check_symbols_list(&fvid_symbols_list, global_variables);
                if check_response.0 == "" {  
                    let mut x_or_y_exists: bool = false;
                    for fvid_symbol in &fvid_symbols_list {
                        let symbol_str: &str = fvid_symbol.symbol();
                        if symbol_str == "x" || symbol_str == "y" {
                            x_or_y_exists = true;
                        }
                    }
                    if x_or_y_exists {
                        let new_fvid: FVID = FVID {
                            id: fvid_symbols_list.to_vec()
                        };
                        fvids.push(new_fvid);
                    }
                }
                for i in (0..fvid_symbols_list.len()).rev() {
                    let symbol: &dyn Symbol = fvid_symbols_list[i];
                    if symbol.symbol() != global_variables.symbols_list[global_variables.symbols_list.len() - 1].symbol() {
                        fvid_symbols_list[i] = FVID::get_next_symbol(symbol, global_variables);
                        break;
                    } else if i > 0 {
                        fvid_symbols_list[i] = global_variables.symbols_list[0];
                    } else {
                        not_finished = false;
                    }
                }
            }
        }

        fvids
    }

    fn compute_for_xy(&self, number_of_nodes: i128, x: i128, y: i128, global_variables: &GlobalVariables) -> (i128, i64, i64) {

        let mut variables_to_process: Vec<i128> = Vec::new();
        let graph_parameters: GraphParameters = GraphParameters {
            x: x,
            y: y,
            number_of_nodes: number_of_nodes
        };
        let mut result;
        let mut symbol_str;
        /*let mut compute_function_start_time;
        let mut compute_variable_start_time;
        let mut compute_function_end_time;
        let mut compute_variable_end_time;
        let mut compute_function_diff;
        let mut compute_variable_diff;*/
        let mut compute_function_duration = 0;
        let mut compute_variable_duration = 0;

        //return (1, compute_function_duration, compute_variable_duration);

        for symbol in self.id.iter().rev() {
            symbol_str = symbol.symbol();
            if global_variables.functions_map.contains_key(&symbol_str) {
                //compute_function_start_time = Utc::now().time();
                result = global_variables.functions_map[symbol_str].compute(&variables_to_process);
                /*compute_function_end_time = Utc::now().time();
                compute_function_diff = compute_function_end_time - compute_function_start_time;
                compute_function_duration = 0;
                match compute_function_diff.num_microseconds() {
                    Some(duration) => {
                        compute_function_duration += duration;
                    }
                    None => {}
                }*/
                //variables_to_process = vec![result];
                variables_to_process.clear();
                variables_to_process.push(result);
            } else {
                //compute_variable_start_time = Utc::now().time();
                let result: i128 = global_variables.variables_map[symbol_str].compute(&graph_parameters);
                /*compute_variable_end_time = Utc::now().time();
                compute_variable_diff = compute_variable_end_time - compute_variable_start_time;
                compute_variable_duration = 0;
                match compute_variable_diff.num_microseconds() {
                    Some(duration) => {
                        compute_variable_duration += duration;
                    }
                    None => {}
                }*/
                variables_to_process.insert(0, result);
            }
        }
        return (variables_to_process[0], compute_function_duration, compute_variable_duration);
    }

    pub fn compute(&self, number_of_nodes: i128, global_variables: &GlobalVariables) -> (Graph, i64, i64, i64, i64) {
        
        /*let create_graph_start_time = Utc::now().time();
        let mut array_matrix = [[0 as i128; 32]; 32];
        let mut compute_function_duration = 0;
        let mut compute_variable_duration = 0;
        let mut x_usize: usize;
        let mut c = 0;
        for x in 0..(number_of_nodes - 1) {
            x_usize = x as usize;
            c = 0;
            for y in (x + 1)..number_of_nodes {
            //for y in 0..number_of_nodes - (x + 1) {
                let result = self.compute_for_xy(number_of_nodes, x, y, global_variables);
                array_matrix[x_usize][c] = result.0;
                //array_matrix[x_usize][c] = 1;
                c += 1;
            }
        }

        let create_graph_end_time = Utc::now().time();
        let create_graph_diff = create_graph_end_time - create_graph_start_time;
        let mut create_graph_duration = 0;
        match create_graph_diff.num_microseconds() {
            Some(duration) => {
                create_graph_duration = duration;
            }
            None => {}
        }

        let compute_start_time = Utc::now().time();*/

        let mut matrix: Vec<Vec<i128>> = Vec::new();
        for x in 0..(number_of_nodes - 1) {
            let mut new_vec = Vec::new();
            //c = 0;
            for y in (x + 1)..number_of_nodes {
                let result = self.compute_for_xy(number_of_nodes, x, y, global_variables);
                new_vec.push(result.0);
                //c += 1;
            }
            matrix.push(new_vec);
        }
        let graph: Graph = Graph::create(&matrix);

        /*let compute_end_time = Utc::now().time();
        let compute_diff = compute_end_time - compute_start_time;
        let mut compute_duration = 0;
        match compute_diff.num_microseconds() {
            Some(duration) => {
                compute_duration = duration;
            }
            None => {}
        }*/

        //(graph, create_graph_duration, compute_duration, compute_function_duration, compute_variable_duration)
        (graph, 0, 0, 0, 0)
    }
}