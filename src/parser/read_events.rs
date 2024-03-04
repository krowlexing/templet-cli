use crate::{util::Ordinal, Map};

use super::{write_event::extract, TuiError};

pub fn parse_events<T>(params: &Map, vals: &[T]) -> Result<Ordinal, TuiError> 
    where T: AsRef<str> + Into<String> + Clone
{
    let ordinal = extract::<usize>(params, "ordinal")?;
    Ok(Ordinal(ordinal))
}