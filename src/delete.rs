use diesel::prelude::*;
use diesel::sql_query;
use crate::PooledConn;

pub fn deleterecord(
    database: &str,
    table: &str,
    id: Vec<(String, String)>,
) -> std::result::Result<String, String> {
    //grab second string from tuple
    let mut stmt = String::from("DELETE FROM ");
    stmt.push_str(database);
    stmt.push_str(".");
    stmt.push_str(table);
    stmt.push_str(" WHERE ");
    stmt.push_str("INTERNAL_PRIMARY_KEY");
    stmt.push_str(" in( ");
    for i in 0..id.len() {
        stmt.push_str(&id[i].1);
        if i != id.len() - 1 {
            stmt.push_str(", ");
        }
    }
    stmt.push_str(")");
    println!("{}", stmt);
    Ok(stmt)
}

pub fn droptable(database: &str, table: &str) -> std::result::Result<String, String> {
    //grab second string from tuple
    let mut stmt = String::from("DROP TABLE ");
    stmt.push_str(database);
    stmt.push_str(".");
    stmt.push_str(table);
    println!("{}", stmt);
    Ok(stmt)
}
pub fn exec_statement(conn: &mut PooledConn, stmt: &str) -> std::result::Result<String, String> {
    //grab second string from tuple
    println!("{}", stmt);
    sql_query(stmt)
        .execute(conn)
        .expect("Failed to execute DELETE/DROP statement");
    Ok(String::from("Executed"))
}
pub fn generate_backup(database: &str, table: &str) -> std::result::Result<String, String> {
    //grab second string from tuple
    let mut stmt = String::from("SELECT * INTO OUTFILE '/tmp/");
    stmt.push_str(database);
    stmt.push_str("_");
    stmt.push_str(table);
    stmt.push_str(
        ".csv' FIELDS TERMINATED BY ',' OPTIONALLY ENCLOSED BY '\"' LINES TERMINATED BY '\n' FROM ",
    );
    stmt.push_str(database);
    stmt.push_str(".");
    stmt.push_str(table);
    println!("{}", stmt);
    Ok(stmt)
}
