use std::str::FromStr;

use crate::Map;
use crate::parser::{ WriteEvent, TuiError, Data };
use crate::util::Tag;

use super::TuiResult;

pub fn parse_event<T>(params: &Map, values: &[T]) -> Result<WriteEvent, TuiError> 
    where T: AsRef<str> + Into<String> + Clone
{
    let tag = Tag(extract::<usize>(params, "tag")?);
    
    let data = parse_data(params, values)?;

    Ok(WriteEvent::new(tag, data))
}

pub fn parse_data<T>(params: &Map, values: &[T]) -> Result<Data, TuiError> 
where T: AsRef<str> + Into<String> + Clone
{
    let filename = nand(
        extract_filename(values),
        extract_literal(params)
    )?;

    let literal = nand(
        extract_literal(params),
        extract_filename(values)
    )?;

    if let Ok(filename) = filename {
        Ok(Data::File(filename))
    } else if let Ok(literal) = literal {
        Ok(Data::Literal(literal))
    } else {
        Err(TuiError::NoParam("filename"))
    }
}
type ArgResult = Result<String, TuiError>;
fn nand(a: ArgResult, b: ArgResult) -> Result<ArgResult, TuiError>{
    if a.is_ok() && b.is_ok() {
        Err(TuiError::Conflict("filename", "literal"))
    } else {
        Ok(a)
    }
}

fn extract_filename<T>(values: &[T]) -> TuiResult<String> 
    where T: AsRef<str> + Into<String> + Clone
{
    if !values.is_empty() {
        Ok(values[0].clone().into())
    } else {
        Err(TuiError::NoParam("filename"))
    }
}

fn extract_literal(params: &Map) -> Result<String, TuiError> 
{
    extract::<String>(params, "string_content")
}

pub fn extract<T: FromStr>(params: &Map, key: &'static str) -> Result<T, TuiError> {
    params.get(key).ok_or(TuiError::NoParam(key))?
        .parse::<T>()
        .map_err(|_| TuiError::WrongValue(key, "integer"))
}