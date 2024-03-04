use crate::Map;

use super::{param, TuiError, TuiResult};

pub enum Execution {
    Sqlite(Sqlite),
    Http(Http)
}

pub struct Sqlite {
    pub path: String,
    pub name: String,
}

pub struct Http {
    pub host: String,
    pub token: String
}

pub fn parse_execution_detail(params: &Map) -> TuiResult<Execution> {
    let http = http(params).map(Execution::Http);
    let sqlite = sqlite(params).map(Execution::Sqlite);

    if http.is_ok() { http }
    else if sqlite.is_ok() || sqlite.as_ref().is_err_and(|e| matches!(e, SqliteTry::NeedName(_))) { 
        sqlite.map_err(|err| err.unwrap()) 
    }
    else { http }
}

pub fn sqlite(params: &Map) -> Result<Sqlite, SqliteTry> {
    let sqlite = param(params, "sqlite").map_err(SqliteTry::None)?;
    let name = param(params, "name").map_err(SqliteTry::NeedName)?;

    Ok(Sqlite { path: sqlite, name })
}

pub enum SqliteTry {
    None(TuiError),
    NeedName(TuiError)
}

impl SqliteTry {
    pub fn unwrap(self) -> TuiError {
        match self {
            SqliteTry::None(v) => v,
            SqliteTry::NeedName(v) => v,
        }
    } 
}

pub fn http(params: &Map) -> TuiResult<Http> {
    let host = param(params, "host")?;
    let token = param(params, "token")?;

    Ok(Http { host, token })
}