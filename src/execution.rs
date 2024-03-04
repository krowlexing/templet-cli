use crate::sqlite::SqliteDb;
use crate::parser::{execution::{Execution, Http, Sqlite}, Action, ActionResult};
use crate::http::{get_events, post_event, post_query, post_reply};

pub fn execute_action(action: Action, execution: Execution) -> Result<ActionResult, ()>  {
    if let Execution::Sqlite(sqlite) = execution {
        execute_sqlite(action, sqlite).map_err(|_| ())
    } else if let Execution::Http(http) = execution {
        Ok(execute_http(action, http).unwrap())    } else {
        Err(())
    }
}

pub fn execute_sqlite(action: Action, sqlite: Sqlite) -> Result<ActionResult, rusqlite::Error> {
    let dbfile = sqlite.path;
    let db = SqliteDb::new(dbfile)?;
    let res = match action {
        Action::WriteEvent(event) => ActionResult::WriteEvent(db.insert(event.name(sqlite.name))?),
        Action::WriteQuery(event) => ActionResult::WriteQuery(db.insert(event.name(sqlite.name))?),
        Action::ReadEvents(ordinal) => ActionResult::ReadEvents(db.read_from(ordinal)?),
        Action::Reply(reply) => ActionResult::Reply(db.answer(reply.event_ordinal, reply.data.bytes().unwrap()).is_ok().to_string())
    };

    Ok(res)
}

pub fn execute_http(action: Action, http: Http) -> Result<ActionResult, reqwest::Error> {
    
    let res = match action {
        Action::WriteEvent(event) => ActionResult::WriteEvent(post_event(http.host, event, http.token)?),
        Action::WriteQuery(event) => ActionResult::WriteQuery(post_query(http.host, event, http.token)?),
        Action::ReadEvents(ordinal) => ActionResult::ReadEvents(get_events(http.host, ordinal, http.token)?),
        Action::Reply(reply) => ActionResult::Reply(post_reply(http.host, reply, http.token)?),
    };

    Ok(res)
}
//WriteEvent(event) {
// 
//}