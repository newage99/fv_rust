pub trait Command {
    fn text() -> String;
    fn run(parameters: Vec<&str>);
    fn help() -> String;
}