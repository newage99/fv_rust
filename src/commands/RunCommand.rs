use super::super::commands::Command::Command;
use super::super::FVID::FVID;

pub struct RunCommand;

impl Command for RunCommand {
    fn text() -> String {
        return String::from("Run");
    }
    fn run(parameters: Vec<&str>) {
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
        FVID::check(id);
    }
    fn help() -> String {
        return String::from("Executes algorithm given parameters.");
    }
}