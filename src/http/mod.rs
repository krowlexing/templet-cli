use std::io::Read;

use reqwest::blocking;

use crate::{parser::{Reply, WriteEvent}, util::{Event, Ordinal}};

pub fn post_event(host: String, event: WriteEvent, token: String) -> Result<Ordinal, reqwest::Error> {
    let client = reqwest::blocking::Client::new();
    
    let str = serde_json::to_string(&event.event()).unwrap();

    let url = format!("http://{}/event", host);
    let x = client.post(url)
        .header("Authorization", token)
        .header("Content-Type", "application/json")
        .body(str)
        .send();

    let mut string = String::new();
    x.unwrap().read_to_string(&mut string).unwrap();

    Ok(serde_json::from_str(&string).unwrap())
}

pub fn post_query(host: String, event: WriteEvent, token: String) -> Result<Ordinal, reqwest::Error> {
    let str = serde_json::to_string(&event.query()).unwrap();

    let x = blocking::Client::new()
        .post(format!("http://{}/event", host))
        .header("Authorization", token)
        .header("Content-Type", "application/json")
        .body(str)
        .send();

    let mut string = String::new();
    x.unwrap().read_to_string(&mut string).unwrap();

    Ok(serde_json::from_str(&string).unwrap())
}

pub fn post_reply(host: String, event: Reply, token: String) -> Result<String, reqwest::Error> {
    let bytes = event.data.bytes().unwrap();

    let x = blocking::Client::new()
        .post(format!("http://{}/event/{}/reply", host, event.event_ordinal.0))
        .header("Authorization", token)
        .body(bytes)
        .send();

    let mut string = String::new();
    x.unwrap().read_to_string(&mut string).unwrap();

    Ok(string)
}

pub fn get_events(host: String, ordinal: Ordinal, token: String) -> Result<Vec<Event>, reqwest::Error> {
    let client = reqwest::blocking::Client::new();
    
    let url = format!("http://{}/events", host);
    client.get(url)
        .query(&[("ord", ordinal)])
        .header("Authorization", token)
        .send()
        .map_err(|e| { println!("{}", e);e })
        .and_then(|r| r.text() )
        .map(|s| serde_json::from_str::<Vec<Event>>(&s).unwrap() )
}