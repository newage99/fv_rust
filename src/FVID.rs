use super::globals::GlobalVariables;

pub struct FVID;

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

impl FVID {
    pub fn check(fvid: &str, global_variables: GlobalVariables) {
        //println!("{}", fvid);
        //println!("{}", symbols.len());
        /*for symbol in &global_variables.symbols_list {
            println!("{}", symbol.symbol());
        }*/
        for key in global_variables.functions_map.keys() {
            println!("{}", key);
        }
        for key in global_variables.variables_map.keys() {
            println!("{}", key);
        }
        for symbol in &global_variables.symbols_list {
            println!("{}", symbol.symbol());
        }
    }
}