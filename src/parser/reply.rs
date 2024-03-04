use crate::{util::Ordinal, Map};

use super::{write_event::{extract, parse_data}, Data, Reply, TuiError};

pub fn parse_reply<T>(params: &Map, vals: &[T]) -> Result<Reply, TuiError> 
    where T: AsRef<str> + Into<String> + Clone
{
    let event_ordinal = extract::<usize>(params, "ordinal").map(Ordinal)?;
    let data = parse_data(params, vals)?;
    Ok(Reply { event_ordinal, data })
}
