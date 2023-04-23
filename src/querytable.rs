use mysql::prelude::*;
use mysql::*;
use crate::pushdata::gettablecol;
pub struct  Querydata<T>{
    data:Vec<T>
}
pub fn query_tables(table: &str, conn: &mut PooledConn, whereclause: &str, database: &str)-> Vec<Vec<String>>{
    let mut query= String::from("SELECT * FROM ");
    query.push_str(table);
    if whereclause != "" {
        query.push_str(" WHERE ");
        query.push_str(whereclause);
    }
    let columns = gettablecol::get_table_col(conn,table, database).unwrap();

    let columntypes = grab_columntypes(conn, table, database).unwrap();


    let columndata=vec![columns,columntypes];
    //query table with columns in columns vector and type in columntypes vector

    columndata
    
}
struct Columntype{
    column:String,
    datatype:String
}
fn grab_columntypes(conn: &mut PooledConn, table: &str, database: &str) -> std::result::Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut query = String::from("SELECT COLUMN_TYPE FROM INFORMATION_SCHEMA.COLUMNS WHERE TABLE_SCHEMA = '");
    query.push_str(database);
    query.push_str("' AND TABLE_NAME = '");
    query.push_str(table);
    query.push_str("'");
    let mut column: String = String::new();
    let mut datatype: String = String::new();
    let mut stmt: Vec<String> = conn.query_map(query, |datatype|datatype)?; //??

    Ok(stmt)
}
