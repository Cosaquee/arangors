use pretty_assertions::{assert_eq, assert_ne};
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;

use arangors::{AqlQuery, Connection, Document};

const URL: &str = "http://localhost:8529/";

#[derive(Deserialize, Debug)]
struct User {
    pub username: String,
    pub password: String,
}

#[test]
fn setup() {
    env_logger::init();
}

#[test]
fn test_aql_str() {
    let conn = Connection::establish_jwt(URL, "root", "KWNngteTps7XjrNv").unwrap();
    let db = conn.db("test_db").unwrap();
    let result: Vec<Document<User>> = db
        .aql_str(r#"FOR i in test_collection FILTER i.username=="test2" return i"#)
        .unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].document.password, "test2_pwd");
}

#[test]
fn test_aql() {
    let conn = Connection::establish_jwt(URL, "root", "KWNngteTps7XjrNv").unwrap();
    let db = conn.db("test_db").unwrap();
    let aql = AqlQuery::new(r#"FOR i in test_collection FILTER i.username=="test2" return i"#);
    let result: Vec<Document<User>> = db.aql_query(aql).unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].document.password, "test2_pwd");
}

#[test]
fn test_aql_bind_vars() {
    let conn = Connection::establish_jwt(URL, "root", "KWNngteTps7XjrNv").unwrap();
    let db = conn.db("test_db").unwrap();
    let aql = AqlQuery::new(r#"FOR i in test_collection FILTER i.username==@username return i"#)
        .bind_var("username", "test2");
    let result: Vec<Document<User>> = db.aql_query(aql).unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].document.password, "test2_pwd");
}
