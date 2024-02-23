use std::{collections::HashMap, str::FromStr};

use reqwest::{blocking::*, Method, Url};

const endpoint: &str = "localhost:11037";

//templet event -l "insert 3 forward" --tag mytag;
//templet event insert.txt --tag mytag;

pub fn event(params: HashMap<String, String>) {
    let client = Client::new();

    client.post(endpoint).send().unwrap();
}