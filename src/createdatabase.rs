use crate::connkey;
use crate::connkey::ApiKey;
use crate::dbconnect;
use diesel::prelude::*;
use diesel::sql_query;
//create a logical database (schema) in Postgres and register an API key
pub fn create_database(database_name: &str) {
    let mut conn = dbconnect::internalqueryconn();
    let mut key = ApiKey::new();
    key.populatekey(database_name.to_string());

    let hash = connkey::random_password();
    let mut query = String::from("CREATE SCHEMA IF NOT EXISTS ");
    query.push_str(database_name);
    query.push_str(";");

    sql_query(query)
        .execute(&mut conn)
        .expect("Failed to create schema");

    let _ = connkey::insert_apikey(database_name.to_string(), hash);
}

pub fn create_databaseweb(database: &str) -> String {
    let mut conn = dbconnect::internalqueryconn();
    let dbname = String::from(database);
    let hash = connkey::random_password();
    let mut query = String::from("CREATE SCHEMA IF NOT EXISTS ");
    query.push_str(&dbname);
    query.push_str(";");

    sql_query(query)
        .execute(&mut conn)
        .expect("Failed to create schema");

    let apikey = connkey::insert_apikey(dbname.to_string(), hash);
    apikey.unwrap()
}
