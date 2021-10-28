mod symbols;
use symbols::Function::Addition::Addition;
use symbols::Symbol::Symbol;

fn main() {
    println!("{}", Addition::symbol());
}