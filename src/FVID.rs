use super::globals::GlobalVariables;
use super::symbols::Function::Function;

pub struct FVID;

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn return_min_variables_not_satisfied_error(symbol_str: &str, min_variables_needed: &str) -> String {
    let mut returnStr: String = String::from("Function '");
    returnStr.push_str(symbol_str);
    returnStr.push_str("' cannot process less than ");
    returnStr.push_str(min_variables_needed);
    returnStr.push_str(" variables.");
    return returnStr;
}

fn check_number_of_variables(function: &dyn Function, number_of_variables_processesed: i16) -> String {
    if number_of_variables_processesed + 1 < function.min_variables_needed() {
        
    }
    return String::new();
}

impl FVID {
    pub fn check(fvid: &str, global_variables: GlobalVariables) -> String {
        let split: Vec<&str> = fvid.split(" ").collect();
        let mut first: bool = true;
        let mut number_of_variables_processesed: i16 = 0;
        let mut last_function: Option<&dyn Function> = None;
        for fvunit in split {
            if !global_variables.symbols_map.contains_key(fvunit) {
                let mut returnStr: String = String::from("Not recognized symbol '");
                returnStr.push_str(fvunit);
                returnStr.push_str("'.");
                return returnStr;
            }
            let is_a_function = global_variables.functions_map.contains_key(fvunit);
            if first {
                if !is_a_function {
                    let mut returnStr: String = String::from("FVID must start with a function. '");
                    returnStr.push_str(fvunit);
                    returnStr.push_str("' is not a function.");
                    return returnStr;
                }
                first = false;
            }
            if is_a_function {
                match last_function {
                    Some(l_function) => {
                        let min_variables_needed: i16 = l_function.min_variables_needed();
                        if number_of_variables_processesed + 1 < min_variables_needed {
                            return return_min_variables_not_satisfied_error(l_function.symbol(), &min_variables_needed.to_string().to_owned());
                        }
                    }
                    None => {}
                }
                last_function = Some(global_variables.functions_map[fvunit]);
                number_of_variables_processesed = 0;
            } else {
                number_of_variables_processesed += 1;
            }
        }
        match last_function {
            Some(l_function) => {
                let min_variables_needed: i16 = l_function.min_variables_needed();
                if number_of_variables_processesed < min_variables_needed {
                    return return_min_variables_not_satisfied_error(l_function.symbol(), &min_variables_needed.to_string().to_owned());
                }
            }
            None => {}
        }
        return String::new();
    }
}