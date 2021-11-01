use super::symbols::Symbol::Symbol;
use super::symbols::Function::Function;
use super::symbols::Variable::Variable;
use std::collections::HashMap;

/*pub struct Global {
    symbolsList: Vec<Box<dyn Symbol>>,
    symbolsMap: HashMap<String, Box<dyn Symbol>>
}

impl Global {
    pub fn loadSymbols(&self) {
        self.symbolsList = vec![Box::new(Addition), Box::new(One)];
        self.symbolsMap = HashMap::new();
        for &symbol in self.symbolsList.iter() {
            let a = String::from(symbol.symbol());
            self.symbolsMap.insert(a, symbol);
        }
    }
}*/

pub struct GlobalVariables {
    pub symbols_list: Vec<&'static dyn Symbol>,
    pub symbols_map: HashMap<&'static str, &'static dyn Symbol>,
    pub functions_list: Vec<&'static dyn Function>,
    pub functions_map: HashMap<&'static str, &'static dyn Function>,
    pub variables_list: Vec<&'static dyn Variable>,
    pub variables_map: HashMap<&'static str, &'static dyn Variable>
}

/*fn unbox<T>(value: Box<T>) -> T {
    *value
}*/

//lazy_static! {
    /*pub static ref symbolsList: Vec<Box<dyn Symbol>> = {
        let mut l = Vec::new();
        let a = Addition;
        let b = a as Symbol;
        l.push(Box::new(a));
        l
    };*/
    /*pub static ref symbolsMap: HashMap<String, Box<dyn Symbol>> = {
        let mut m: HashMap<String, Box<dyn Symbol>> = HashMap::new();
        for symbol in symbolsList.iter() {
            let a = String::from(symbol.symbol());
            //m.insert(a, unbox(Box::new(symbol)));
            //m.insert(a, *symbol);
        }
        m
    };*/
//}