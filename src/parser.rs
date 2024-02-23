use std::collections::HashMap;

pub struct ArgsDescription {
}

// Flag
// Value
// ManyValues

// Positionals

pub struct FlagArg {
    key: String,
    long: &'static [&'static str],
    short: &'static [&'static str],
}

impl Arg for FlagArg {
    fn satisfies(&self, str: &String) -> bool {
        if str.eq(self.long[0]) {
            true
        } else { str.eq(self.short[0]) }
    }

    fn key(&self) -> String {
        self.key.clone()
    }

    fn flag(&self) -> bool {
        true
    }
}


pub fn flag() -> FlagArg {
    FlagArg {
        key: "extra".to_owned(),
        long: &["--extra"],
        short: &["-x"],
    }
}

pub fn base() -> ValueArg {
    ValueArg {
        key: String::from("base"),
        long: &["--base"],
        short: &["-b"],
    }
}

pub struct ValueArg {
    key: String,
    long: &'static [&'static str],
    short: &'static [&'static str],
}

impl Arg for ValueArg {
    fn satisfies(&self, str: &String) -> bool {
        if str.eq(self.long[0]) {
            true
        } else { str.eq(self.short[0]) }
    }

    fn key(&self) -> String {
        self.key.clone()
    }

    fn flag(&self) -> bool {
        false
    }
}

//check if keys conflict; long, short too

fn hehe() {
    let flag = flag();
}

trait Arg {
    fn satisfies(&self, str: &String) -> bool;
    fn key(&self) -> String;
    fn flag(&self) -> bool;
}

// templet command [--flags...] [target]
// OR
// templet --help

pub fn parse() {
    let mut map: HashMap<String, String> = HashMap::new();
    let mut args = std::env::args();
    let mut rules: Vec<Box<dyn Arg>> = vec!(Box::new(flag()), Box::new(base()) );
    let mut values = vec!();
    'outer: while let Some(arg_string) = args.next() {
        for rule in &rules {
            if rule.satisfies(&arg_string) {
                if rule.flag() {
                    map.insert(rule.key(), "x".to_owned());
                } else {
                    map.insert(rule.key(), args.next().expect("flag expected value"));
                }
                continue 'outer;
            }
        }
        values.push(arg_string.clone());
    }

    print_values(values);
    print_map(map);
}

fn print_values(values: Vec<String>) {
    println!("\nvalues\n");
    for value in values {
        println!("{}", value);
    }
}

fn print_map(map: HashMap<String, String>) {
    println!("\nmap contents\n");
    for entry in map {
        println!("{} - {}", entry.0, entry.1);
    }
}