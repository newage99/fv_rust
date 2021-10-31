use super::globals::GlobalVariables;

pub struct FVID;

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

impl FVID {
    pub fn check(fvid: &str, global_variables: GlobalVariables) -> String {
        let split: Vec<&str> = fvid.split(" ").collect();
        let mut first: bool = true;
        let number_of_variables_processesed = 0;
        for fvunit in split {
            if !global_variables.symbols_map.contains_key(fvunit) {
                let mut returnStr: String = String::from("Not recognized symbol '");
                returnStr.push_str(fvunit);
                returnStr.push_str("'.");
                return returnStr;
            }
            if first {
                if !global_variables.functions_map.contains_key(fvunit) {
                    let mut returnStr: String = String::from("FVID must start with a function. '");
                    returnStr.push_str(fvunit);
                    returnStr.push_str("' is not a function.");
                    return returnStr;
                }
                first = false;
            }
            
        }
        for key in global_variables.functions_map.keys() {
            println!("{}", key);
        }
        for key in global_variables.variables_map.keys() {
            println!("{}", key);
        }
        for symbol in &global_variables.symbols_list {
            println!("{}", symbol.symbol());
        }
        return String::new();
    }
}