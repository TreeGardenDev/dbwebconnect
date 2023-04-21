use mysql::prelude::*;
use mysql::*;
use serde::{Deserialize, Serialize};
pub fn query_tables(table: &str, conn: &mut PooledConn, whereclause: &str)-> Result<Vec<String>> {
    let mut tables = Vec::new();
    let mut query= String::from("SELECT * FROM ");
    query.push_str(table);
    if whereclause != "" {
        query.push_str(" WHERE ");
        query.push_str(whereclause);
    }
    let result=conn.query(query).unwrap();
    for row in result {
        tables.push(
            row    
        );
    }
    Ok(tables)
}
