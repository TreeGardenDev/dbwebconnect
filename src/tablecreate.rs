use mysql::prelude::*;
use mysql::*;
use csv::Reader;
use clap::Parser;
//read from csv file to create table in mariadb with given column names
pub fn create_table(conn: &mut PooledConn, table_name: &str, column_names: &Vec<String>, column_types: &Vec<String> ){
    let mut query = String::from("CREATE TABLE ");
    query.push_str(table_name);
    query.push_str(" (");
    for i in 0..column_names.len() {
        query.push_str(column_names[i].as_str());
        query.push_str(" ");
        query.push_str(column_types[i].as_str());
    }
    query.pop();
    query.push_str(")");
    conn.query_drop(query).unwrap();
}
