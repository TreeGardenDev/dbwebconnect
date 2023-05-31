use mysql::prelude::*;
use crate::dbconnect;
use crate::connkey;
//create a mysql database from a csv file
pub fn create_database(database_name: &str) {
    //let mut conn = mysql::Conn::new("mysql://kylelocal:kcb@127.0.0.1:3306/").unwrap();
    let mut conn=dbconnect::database_connection_no_db();
    let query = format!("CREATE DATABASE IF NOT EXISTS {}", database_name);
    conn.query_drop(query).unwrap();

    let _=connkey::insert_apikey(database_name.to_string());
}

pub fn create_databaseweb(database: &str,database_user: &str,database_password: &str, database_host: &str, port: &str) {
    //let mut conn = mysql::Conn::new("mysql://kylelocal:kcb@127.0.0.1:3306/").unwrap();
    let dbname=String::from(database);
    let mut conn=dbconnect::database_connection_no_db_web(database_user,database_password,database_host,port);
    let query = format!("CREATE DATABASE IF NOT EXISTS {}", dbname);
    conn.query_drop(query).unwrap();
}
