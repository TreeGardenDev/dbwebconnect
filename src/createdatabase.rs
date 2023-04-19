use mysql::prelude::*;
use mysql::*;
use csv::Reader;

//create a mysql database from a csv file
pub fn create_database(database_name: &str) {
    let mut conn = mysql::Conn::new("mysql://root:kcb@localhost").unwrap();
    let query = format!("CREATE DATABASE IF NOT EXISTS {}", database_name);
    conn.query_drop(query).unwrap();
}
