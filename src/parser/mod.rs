use std::io::{self, Read};
use std::fs::File;
use std::fmt::{self, Display};
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{Map, OptionExt};
use crate::util::{Event, NewEvent, NewHttpEvent, Ordinal, Tag};
use self::reply::parse_reply;
use self::write_event::parse_event;
use self::read_events::parse_events;
use self::execution::{parse_execution_detail, Execution};
use self::arg::{flag, Arg, FlagArg, ValueArg};

pub mod execution;
pub mod read_events;
mod write_event;
mod reply;
mod arg;

pub enum TuiError {
    Conflict(&'static str, &'static str),
    NoParam(&'static str),
    NoAction(),
    WrongValue(&'static str, &'static str)
}

impl Display for TuiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let TuiError::Conflict(param1, param2) = self {
            f.write_fmt(format_args!("Can't use param '{}' with '{}'", param1, param2))?;
        } else if let TuiError::NoParam(param) = self {
            f.write_fmt(format_args!("Required parameter '{}' is not supplied", param))?;
        } else if let TuiError::WrongValue(param, expected_type) = self {
            f.write_fmt(format_args!("Parameter '{}' must be valid {}", param, expected_type))?;
        } else if let TuiError::NoAction() = self {
            f.write_fmt(format_args!("Action verb is required.\n  Available actions: [event, query, events, reply]"))?;
        }
        Ok(())
    }
}

pub type TuiResult<T> = Result<T, TuiError>;

pub fn parse() -> (Map, Vec<String>) {
    let mut map: HashMap<String, String> = HashMap::new();
    let mut args = std::env::args();
    let rules: Vec<Box<dyn Arg>> = vec!(
        Box::new(flag()),
        Box::new(FlagArg {
            key: "quiet",
            long: &["quiet"],
            short: &["q"]
        }),
        ValueArg::build()
            .key("tag")
            .long("tag")
            .short("t")
            .boxed(),
        ValueArg::build()
            .key("ordinal")
            .long("ord")
            .short("o")
            .boxed(),
        ValueArg::build()
            .key("string_content")
            .long("string_content")
            .short("s")
            .boxed(),
        ValueArg::build()
            .key("sqlite")
            .long("sqlite")
            .short("S")
            .boxed(),
        ValueArg::build()
            .key("name")
            .long("name")
            .short("n")
            .boxed(),
        ValueArg::build()
            .key("host")
            .long("host")
            .short("h")
            .boxed(),
        ValueArg::build()
            .key("token")
            .long("token")
            .short("T")
            .boxed()
    );
    let mut values = vec!();
    
    while let Some(arg_string) = args.next() {
        rules.iter()
            .find(|r| r.satisfies(&arg_string))
            .on_none(|| values.push(arg_string.clone()))
            .apply(|rule| {
                if rule.flag() {
                    map.insert(rule.key(), "x".to_owned());
                } else {
                    map.insert(rule.key(), args.next().expect("flag expected value"));
                }
            });
    }

    (map, values)
}

pub fn parse_args<T>(params: &Map, values: &[T]) -> TuiResult<(Action, Execution)> 
    where T: AsRef<str> + PartialEq<str> + Into<String> + Clone
{
    let execution = parse_execution_detail(params);

    if values.is_empty() { return Err(TuiError::NoAction()) }

    let action_word = &values[0];

    if action_word == "event" {
        let action = parse_event(params, &values[1..]).map(Action::WriteEvent);
        join(action, execution)
    } else if action_word == "query" {
        let action = parse_event(params, &values[1..]).map(Action::WriteQuery);
        join(action, execution)
    } else if action_word == "events" {
        let action = parse_events(params, &values[1..]).map(Action::ReadEvents);
        join(action, execution)
    } else if action_word == "reply" {
        let action = parse_reply(params, &values[1..]).map(Action::Reply);
        join(action, execution)
    } else {
        Err(TuiError::NoAction())
    }
}

pub fn join(action: TuiResult<Action>, execution: TuiResult<Execution>) -> TuiResult<(Action, Execution)> {
    let action = action?;
    let execution = execution?;

    Ok((action, execution))
}

pub fn param(params: &Map, key: &'static str) -> TuiResult<String> {
    params.get(key).ok_or(TuiError::NoParam(key)).map(|str| str.clone())
}

pub enum Action {
    WriteEvent(WriteEvent),
    WriteQuery(WriteEvent),
    ReadEvents(Ordinal),
    Reply(Reply)
}

pub enum ActionResult {
    WriteEvent(Ordinal),
    WriteQuery(Ordinal),
    ReadEvents(Vec<Event>),
    Reply(String)
}

impl Display for ActionResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use ActionResult::*;

        match self {
            WriteEvent(ord) => f.write_fmt(format_args!("{}", ord.0)),
            WriteQuery(ord) => f.write_fmt(format_args!("{}", ord.0)),
            Reply(reply) => f.write_fmt(format_args!("{}", reply)),
            ReadEvents(events) => {
                let mut res = Result::Ok(());
                for event in events {
                    res = res.and(f.write_fmt(format_args!("{}\n", event)));
                }
                res
            },
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct WriteEvent {
    tag: Tag,
    data: Data
}

impl WriteEvent {
    pub fn new(tag: Tag, data: Data) -> WriteEvent {
        WriteEvent { tag, data }
    }

    pub fn name(self, name: String) -> NewEvent {
        NewEvent { tag: self.tag, external: false, name, data: self.data.bytes().unwrap() }
    }

    pub fn event(self) -> NewHttpEvent {
        NewHttpEvent {
            tag: self.tag,
            external: false,
            data: self.data.bytes().unwrap()
        }
    }

    pub fn query(self) -> NewHttpEvent {
        NewHttpEvent {
            tag: self.tag,
            external: true,
            data: self.data.bytes().unwrap(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Reply {
    pub event_ordinal: Ordinal,
    pub data: Data
}

#[derive(serde::Deserialize, serde::Serialize)]
pub enum Data {
    File(String),
    Literal(String),
}

impl Data {
    pub fn bytes(self) -> Result<Vec<u8>, io::Error> {
        match self {
            Data::Literal(string) => Ok(string.into()),
            Data::File(filename) => {
                let mut string = String::new();
                File::open(filename)?.read_to_string(&mut string)?;
                Ok(string.into())
            },
        }
    }
}