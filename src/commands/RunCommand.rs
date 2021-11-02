use super::super::commands::Command::Command;
use super::super::FVID::FVID;
use super::super::globals::GlobalVariables;
use super::super::Graph::Graph;

pub struct RunCommand;

impl Command for RunCommand {
    fn text() -> String {
        return String::from("Run");
    }
    fn run(parameters: Vec<&str>, global_variables: GlobalVariables) {
        let mut id: &str = "";
        for param in &parameters {
            if param.contains("=") {
                let split = param.split("=").collect::<Vec<&str>>();
                if split.len() > 1 {
                    let identifier = split[0];
                    let value = split[1];
                    if identifier == "id" {
                        id = value;
                    }
                }
            }
        }
        if id == "" {
            println!("'id' parameter not provided.");
            return;
        }
        let response = FVID::check(id, &global_variables);
        if response != "" {
            println!("'{}' check: {}", id, response);
        }
        let fvid: FVID = FVID::create(id, &global_variables);
        let graph: Graph = fvid.compute(4, &global_variables);
        graph.print();
    }
    fn help() -> String {
        return String::from("Executes algorithm given parameters.");
    }
}