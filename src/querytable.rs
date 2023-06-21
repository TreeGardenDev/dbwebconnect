use mysql::prelude::*;
use mysql::*;
use serde_json::json;
use serde_json::Value;
pub mod displayquery;
pub fn query_tables(
    table: &str,
    conn: &mut PooledConn,
    whereclause: &str,
    database: &str,
    select: Vec<&str>,
) -> Vec<Vec<String>> {
    //let columns = gettablecol::get_table_col(conn,table, database, select).unwrap();
    
    let columns_stmt = grab_columnnames(table, database, select).unwrap();
    let columns = exec_map(conn, &columns_stmt).unwrap();

    //let columntypes = grab_columntypes(conn, table, database).unwrap();

    let querydata = query_table(conn, table, whereclause, database, columns).unwrap();

    //columndata
    querydata
}

pub fn exec_map(
    conn: &mut PooledConn,
    query: &str,
) -> std::result::Result<Vec<String>, Box<dyn std::error::Error>> {
    let stmt: Vec<String> = conn.query_map(query, |data| data)?;
    Ok(stmt)
}
pub fn grab_columntypes(
    table: &str,
    database: &str,
) -> std::result::Result<String, Box<dyn std::error::Error>> {
    let mut query =
        String::from("SELECT COLUMN_TYPE FROM INFORMATION_SCHEMA.COLUMNS WHERE TABLE_SCHEMA = '");
    query.push_str(database);
    query.push_str("' AND TABLE_NAME = '");
    query.push_str(table);
    query.push_str("'");
    query.push_str("And COLUMN_NAME != 'INTERNAL_PRIMARY_KEY'");

    Ok(query)
    //let stmt: Vec<String> = conn.query_map(query, |datatype|datatype)?; //??
    //Ok(stmt)
}
pub fn grab_columnnames(
    table: &str,
    database: &str,
    select: Vec<&str>,
) -> std::result::Result<String, Box<dyn std::error::Error>> {
    let mut query =
        String::from("SELECT COLUMN_NAME FROM INFORMATION_SCHEMA.COLUMNS WHERE TABLE_SCHEMA = '");
    query.push_str(database);
    query.push_str("' AND TABLE_NAME = '");
    query.push_str(table);
    query.push_str("'");
    query.push_str("And COLUMN_NAME != 'INTERNAL_PRIMARY_KEY'");
    if select[0]!="*"{
        query.push_str("And COLUMN_NAME in ( ");
        for i in 0..select.len() {
            query.push_str("'");
            query.push_str(select[i]);
            query.push_str("'");
            if i != select.len() - 1 {
                query.push_str(", ");
            }
        }
        query.push_str(")");
    }
    //let stmt: Vec<String> = conn.query_map(query, |datatype|datatype)?; //??
    Ok(query)
}

fn query_table(
    conn: &mut PooledConn,
    table: &str,
    whereclause: &str,
    database: &str,
    columntypes: Vec<String>,
) -> std::result::Result<Vec<Vec<String>>, Box<dyn std::error::Error>> {
    //    let stmt: Vec<Vec<String>> = conn.query_map(query, |(col1)|{
    //        let mut row: Vec<String> = Vec::new();
    //        row.push(col1);
    //    })?; //??

    let mut stmt = Vec::new();
    for i in 0..columntypes.len() {
        let mut query = String::from("SELECT ");
        query.push_str(&columntypes[i]);
        query.push_str(" FROM ");
        query.push_str(database);
        query.push_str(".");
        query.push_str(table);
        if whereclause != "" {
            query.push_str(" WHERE ");
            query.push_str(whereclause);
        }
        let row = conn
            .query_map(query.clone(), |columntypes: String| columntypes)
            .unwrap();

        stmt.push(row);
        query.clear();
    }
    //let mut fixedstmt:Vec<Vec<String>>=vec![&columntypes.len(), &stmt.len()];
    Ok(stmt)
}
pub fn build_json(
    queryresult: Vec<Vec<String>>,
    database: &str,
    table: &str,
    conn: &mut PooledConn,
    select: Vec<&str>,
) -> Value {
    //let columns = gettablecol::get_table_col(conn,table, database).unwrap();
    let columns_stmt = grab_columnnames(table, database, select).unwrap();
    let columns = exec_map(conn, &columns_stmt).unwrap();
    let mut recordcount = 0;
    if let Some(row) = queryresult.get(1) {
        recordcount = row.len();
        println!("recordcount: {}", recordcount);
    }

    let mut jsondata = json!({});
    for x in 0..recordcount {
        let mut jsonarray = json!({});
        for i in 0..queryresult.len() {
            jsonarray[&columns[i]] = queryresult[i][x].clone().into();
        }
        jsondata[&x.to_string()] = jsonarray;
    }
    jsondata
}
pub fn query_table_schema(
    columns: Vec<String>,
    types: Vec<String>,

)-> Value {
    let mut jsondata = json!({});
    for x in 0..columns.len() {
        let mut jsonarray = json!({});
        jsonarray["column_name"] = columns[x].clone().into();
        jsonarray["column_type"] = types[x].clone().into();
        jsondata[&x.to_string()] = jsonarray;
    }
    jsondata

}
