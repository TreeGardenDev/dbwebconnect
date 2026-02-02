use crate::dbconnect;
use diesel::prelude::*;
use diesel::sql_query;
//create a logical database (schema) in Postgres and register an API key
pub fn create_database(database_name: &str) {
    let mut conn = dbconnect::internalqueryconn();
    let mut query = String::from("CREATE SCHEMA IF NOT EXISTS ");
    query.push_str(database_name);
    query.push_str(";");

    sql_query(query)
        .execute(&mut conn)
        .expect("Failed to create schema");
}

pub fn create_databaseweb(database: &str) -> String {
    let mut conn = dbconnect::internalqueryconn();
    let dbname = String::from(database);
    let mut query = String::from("CREATE SCHEMA IF NOT EXISTS ");
    query.push_str(&dbname);
    query.push_str(";");

    sql_query(query)
        .execute(&mut conn)
        .expect("Failed to create schema");

    // Legacy behavior returned an API key; JWT auth no longer needs this.
    // Return a simple success marker for compatibility with existing callers.
    String::from("ok")
}
