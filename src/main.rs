mod symbols;
mod commands;
mod FVID;
mod Global;
use Global::symbolsMap;
use commands::Command::Command;
use commands::RunCommand::RunCommand;

fn main() {

    //println!("The entry for `0` is \"{}\".", symbolsMap.get(&0).unwrap().symbol());
    println!("{}", symbolsMap.contains_key("dsad"));

    RunCommand::run(vec!["id=1 + 1"]);
}