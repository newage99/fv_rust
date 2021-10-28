use super::symbols::Symbol::Symbol;
use super::symbols::Function::Addition::Addition;
use super::symbols::Variable::One::One;
use std::collections::HashMap;
use lazy_static::lazy_static;

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

pub static symbolsList: Vec<Box<dyn Symbol>> = vec![];

fn unbox<T>(value: Box<T>) -> T {
    *value
}

lazy_static! {
    /*pub static ref symbolsList: Vec<Box<dyn Symbol>> = {
        let mut l = Vec::new();
        let a = Addition;
        let b = a as Symbol;
        l.push(Box::new(a));
        l
    };*/
    pub static ref symbolsMap: HashMap<String, Box<dyn Symbol>> = {
        let mut m: HashMap<String, Box<dyn Symbol>> = HashMap::new();
        for symbol in symbolsList.iter() {
            let a = String::from(symbol.symbol());
            //m.insert(a, unbox(Box::new(symbol)));
            //m.insert(a, *symbol);
        }
        m
    };
}