pub trait Arg {
    fn satisfies(&self, str: &str) -> bool;
    fn key(&self) -> String;
    fn flag(&self) -> bool;
}
pub struct FlagArg {
    pub key: &'static str,
    pub long: &'static [&'static str],
    pub short: &'static [&'static str],
}

impl Arg for FlagArg {
    fn satisfies(&self, str: &str) -> bool {
        let long_form = format!("--{}", self.long[0]);
        let short_form = format!("-{}", self.short[0]);
        if *str == long_form {
            println!("str: {} long: {}", str, long_form);
            true
        } else { *str == short_form }
    }

    fn key(&self) -> String {
        self.key.to_string()
    }

    fn flag(&self) -> bool {
        true
    }
}


pub fn flag() -> FlagArg {
    FlagArg {
        key: "extra",
        long: &["--extra"],
        short: &["-x"],
    }
}

pub struct ValueArg {
    key: String,
    long: Vec<String>,
    short: Vec<String>,
}

impl ValueArg {
    pub fn build() -> ValueArgBuilder {
        ValueArgBuilder::new()
    }
}

pub struct ValueArgBuilder {
    key: Option<String>,
    long: Vec<String>,
    short: Vec<String>
}

impl ValueArgBuilder {
    pub fn new() -> ValueArgBuilder {
        ValueArgBuilder {
            key: None,
            long: Vec::new(),
            short: Vec::new()
        }
    }

    pub fn key(mut self, key: &str) -> ValueArgBuilder {
        self.key = Some(key.into());
        self
    }

    pub fn long(mut self, long: &str) -> ValueArgBuilder {
        self.long.push(long.into());
        self
    }

    pub fn short(mut self, short: &str) -> ValueArgBuilder {
        self.short.push(short.into());
        self
    }

    pub fn done(self) -> Result<ValueArg, ()> {
        let key = self.key.ok_or(())?;


        Ok(ValueArg { 
            key,
            long: self.long, 
            short: self.short
        })
    }

    pub fn boxed(self) -> Box<ValueArg> {
        Box::new(self.done().expect("key should be set with `Builder.key(some_key)`"))
    }
}

impl Arg for ValueArg {
    fn satisfies(&self, str: &str) -> bool {
        let long_form = format!("--{}", self.long[0]);
        let short_form = format!("-{}", self.short[0]);
        if str == long_form {
            true
        } else { str == short_form }
    }

    fn key(&self) -> String {
        self.key.clone()
    }

    fn flag(&self) -> bool {
        false
    }
}