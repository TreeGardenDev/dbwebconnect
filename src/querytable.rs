use mysql::prelude::*;
use mysql::*;
use serde::{Deserialize, Serialize};
use crate::pushdata::gettablecol;
pub fn query_tables(table: &str, conn: &mut PooledConn, whereclause: &str, database: &str)-> Result<Vec<String>> {
    let mut tables = Vec::new();
    let mut query= String::from("SELECT * FROM ");
    query.push_str(table);
    if whereclause != "" {
        query.push_str(" WHERE ");
        query.push_str(whereclause);
    }
    let columns = gettablecol::get_table_col(conn,table, database).unwrap();
    let query = query.as_str();
    let mut stmt:Vec<String> = conn.query(query)?;
    for row in stmt {
        tables.push(row);
    }

    Ok(tables)
}
