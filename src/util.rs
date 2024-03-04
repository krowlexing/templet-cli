use std::fmt::Display;

use serde::{Deserialize, Serialize};
#[derive(serde::Deserialize, serde::Serialize)]
pub struct Ordinal(pub usize);

impl Display for Ordinal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.0))
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Tag(pub usize);

impl Display for Tag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.0))
    }
}

#[derive(Serialize, Deserialize)]
pub struct Event {
    ordinal: Ordinal,
    tag: Tag,
    ext: bool,
    name: String,
    data: Vec<u8>,
    answer: Option<Answer>,
}

#[derive(Serialize, Deserialize)]
pub struct NewEvent {
    pub tag: Tag,
    pub external: bool,
    pub name: String,
    pub data: Vec<u8>,
}

#[derive(Serialize, Deserialize)]
pub struct NewHttpEvent {
    pub tag: Tag,
    pub external: bool,
    pub data: Vec<u8>,
}

impl Event {
    pub fn from_values(ordinal: Ordinal, tag: Tag, external: bool, name: String, data: Vec<u8>, answer: Option<Answer>) -> Self {
        Self {
            ordinal,
            tag,
            ext: external,
            name,
            data,
            answer,
        }
    }
}

impl Display for Event {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        
        f.write_fmt(format_args!("#{} - tag {}\n  external - [{}], created by {}\n  data: ",
            self.ordinal, self.tag, if self.ext {"x"} else {" "}, self.name
        ))?;

        if let Ok(string) = String::from_utf8(self.data.clone()) {
            f.write_fmt(format_args!("{}", string))?
        } else {
            f.write_fmt(format_args!("{:X?}", &self.data))?
        }

        if let Some(answer) = &self.answer {
            f.write_fmt(format_args!("\n  name: {} answer: ", answer.name))?;
            if let Ok(string) = String::from_utf8(self.data.clone()) {
                f.write_fmt(format_args!("{}", string))?
            } else {
                f.write_fmt(format_args!("{:X?}", &self.data))?
            }
        }

        Ok(())
    }
}

#[derive(Serialize, Deserialize)]
pub struct Answer{
    name: String,
    data: Vec<u8>
}

impl TryFrom<Vec<u8>> for Answer {
    type Error = ();

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        if !value.is_empty() {
            let string_size = value[0] as usize;
            let rest = &value[1..];
            if rest.len() >= string_size {
                let string_bytes = &rest[..string_size];
                let string = String::from_utf8(string_bytes.to_vec()).unwrap();
                let data = value[1 + string_size ..].to_vec();
                Ok(Self { name: string, data})
            } else {
                Err(())
            }
        } else {
            Err(())
        }
    }
}
