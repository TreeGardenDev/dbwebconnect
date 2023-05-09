use mysql::prelude::*;
use crate::dbconnect;
//create a mysql database from a csv file
pub fn create_database(database_name: &str) {
    //let mut conn = mysql::Conn::new("mysql://kylelocal:kcb@127.0.0.1:3306/").unwrap();
    let mut conn=dbconnect::database_connection_no_db();
    let query = format!("CREATE DATABASE IF NOT EXISTS {}", database_name);
    conn.query_drop(query).unwrap();
}
