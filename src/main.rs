use crate::parser::parse;

mod parser;
mod requests;

fn main() {
    parse();
    let (action, args) = parse_args();
    execute_action(action, args);
    println!("Hello, world!");
}

fn parse_args() -> (u32, u32) {
    (0, 0)
}

fn execute_action(action: u32, args: u32) {

}