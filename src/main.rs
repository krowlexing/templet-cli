use std::collections::HashMap;

use execution::execute_action;
use parser::{parse_args, TuiResult};

use crate::parser::parse;

mod parser;
mod util;
mod execution;
mod http;
mod sqlite;

type Map = HashMap<String, String>;

fn main() {
    let result = main_();

    if let Err(error) = result {    
        println!("Error!\n{}", error);
    }
}

fn main_() -> TuiResult<()>{
    let (params, values) = parse();
    
    let (action, args) = parse_args(&params, &values[1..])?;
    let result = execute_action(action, args);

    println!("{}", result.unwrap());
    
    Ok(())
}

trait OptionExt<T> {
    fn apply<F: FnOnce(T)> (self, f: F);

    fn on_none<F: FnOnce()>(self, f: F) -> Option<T>;
}

impl<T> OptionExt<T> for Option<T> {
    fn apply<F: FnOnce(T)> (self, f: F) {
        if let Some(t) = self {
            f(t)
        }
    }

    fn on_none<F: FnOnce()>(self, f: F) -> Option<T> {
        if self.is_none() {
            f();
        }
        self
    }
}