use mysql::prelude::*;
use mysql::*;
use csv::Reader;

//create a mysql database from a csv file
pub fn create_database(database_name: &str) {
    let mut conn = mysql::Conn::new("mysql://kylelocal:kcb@localhost:3306").unwrap();
    let query = format!("CREATE DATABASE IF NOT EXISTS {}", database_name);
    conn.query_drop(query).unwrap();
}
