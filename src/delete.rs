use diesel::prelude::*;
use diesel::sql_query;
use crate::PooledConn;

pub fn deleterecord(
    database: &str,
    table: &str,
    id: Vec<(String, String)>,
) -> std::result::Result<String, String> {
    // Collect and validate numeric IDs make sure positive
    let mut numeric_ids: Vec<i64> = Vec::new();
    for (_, raw) in id.iter() {
        let trimmed = raw.trim().trim_matches('"');
        match trimmed.parse::<i64>() {
            Ok(v) if v > 0 => numeric_ids.push(v),
            _ => return Err("invalid record id".to_string()),
        }
    }

    if numeric_ids.is_empty() {
        return Err("no record ids provided".to_string());
    }

    let mut stmt = String::from("DELETE FROM ");
    stmt.push_str(database);
    stmt.push_str(".");
    stmt.push_str(table);
    stmt.push_str(" WHERE INTERNAL_PRIMARY_KEY IN (");
    for (idx, v) in numeric_ids.iter().enumerate() {
        if idx > 0 {
            stmt.push_str(", ");
        }
        stmt.push_str(&v.to_string());
    }
    stmt.push_str(")");
    println!("{}", stmt);
    Ok(stmt)
}

pub fn droptable(database: &str, table: &str) -> std::result::Result<String, String> {
    // This is only called from admin-only handlers; the database and table
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
