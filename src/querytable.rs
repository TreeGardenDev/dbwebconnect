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
pub fn exec_map_tuple(
    conn: &mut PooledConn,
    query: &str,
) -> std::result::Result<Vec<(String,String)>, Box<dyn std::error::Error>> {
    let stmt: Vec<(String,String)> = conn.query_map(query, |(data1,data2)| (data1,data2))?;
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
    query.push_str("And COLUMN_NAME != 'GPS_ID'");
    query.push_str("And COLUMN_NAME != 'X_COORD'");
    query.push_str("And COLUMN_NAME != 'Y_COORD'");
    query.push_str("And COLUMN_NAME != 'Attachment'");

    Ok(query)
    //let stmt: Vec<String> = conn.query_map(query, |datatype|datatype)?; //??
    //Ok(stmt)
}
pub fn grab_columntypes_schema(
    table: &str,
    database: &str,
) -> std::result::Result<String, Box<dyn std::error::Error>> {
    let mut query =
        String::from("SELECT COLUMN_TYPE FROM INFORMATION_SCHEMA.COLUMNS WHERE TABLE_SCHEMA = '");
    query.push_str(database);
    query.push_str("' AND TABLE_NAME = '");
    query.push_str(table);
    query.push_str("'");

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
    query.push_str("And COLUMN_NAME != 'GPS_ID'");
    query.push_str("And COLUMN_NAME != 'X_COORD'");
    query.push_str("And COLUMN_NAME != 'Y_COORD'");
    query.push_str("And COLUMN_NAME != 'Attachment'");
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
pub fn grab_columnnames_schema(
    table: &str,
    database: &str,
) -> std::result::Result<String, Box<dyn std::error::Error>> {
    let mut query =
        String::from("SELECT COLUMN_NAME FROM INFORMATION_SCHEMA.COLUMNS WHERE TABLE_SCHEMA = '");
    query.push_str(database);
    query.push_str("' AND TABLE_NAME = '");
    query.push_str(table);
    query.push_str("'");
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
    constraints: Vec<(String,String)>,

)-> Value {
    let mut jsondata = json!({});
    println!("Constraints: {:?}", constraints);
    
    for x in 0..columns.len() {
        let mut jsonarray = json!({});
        jsonarray["column_name"] = columns[x].clone().into();
        jsonarray["column_type"] = types[x].clone().into();
        //loop over constraint name to see if it matches column name
        //if constraints[x] != "" {
        //match column name to constraint column if column name is in constraint name
        //
        for y in constraints.iter(){
            if y.0 == columns[x]{
                //check if constraint_name already exists for column
                //if jsonarray["constraint_name"] != "" {
                //    //if constraint_name already exists, append new constraint_name to existing constraint_name
                //    let mut constraint_name = jsonarray["constraint_name"].to_string();
                //    if constraint_name != "" {
                //    constraint_name.push_str(", ");
                //    constraint_name.push_str(&y.1.clone());
                //    jsonarray["constraint_name"] = constraint_name.into();
                //    }
                //}else{

                //    jsonarray["constraint_name"] = y.1.clone().into();
                //}
                jsonarray["constraint_name"] = y.1.clone().into();
            }
        }

        //}
        jsondata[&x.to_string()] = jsonarray;
    }
    jsondata

}
pub fn grab_tablenames(
    database: &str,
) -> std::result::Result<String, Box<dyn std::error::Error>> {
    let mut query = String::from("SELECT TABLE_NAME FROM INFORMATION_SCHEMA.TABLES WHERE TABLE_SCHEMA = '");
    query.push_str(database);
    query.push_str("'");
    Ok(query)
}

pub fn exec_grab_tablenames(
    conn: &mut PooledConn,
    query: &str,
) -> std::result::Result<Vec<String>, Box<dyn std::error::Error>> {
    let stmt: Vec<String> = conn.query_map(query, |data| data)?;
    Ok(stmt)
}

pub fn json_table_names(
    queryresult: Vec<String>,
    database: &str,
) -> Value {
    let mut jsondata = json!({});
    for x in 0..queryresult.len() {
        let mut jsonarray = json!({});
        jsonarray["table_name"] = queryresult[x].clone().into();
        jsonarray["table_schema"] = database.clone().into();
        jsondata[&x.to_string()] = jsonarray;
    }
    jsondata
}
pub fn query_database_schema(
    tablecoltypestorage: Vec<(&str,Vec<String>, Vec<String>, Vec<(String,String)>)>,
    database: &str,
    ) -> Value {
    let mut jsondata = json!({});
    for x in 0..tablecoltypestorage.len() {
        let mut jsonarray = json!({});
        jsonarray["table_name"] = tablecoltypestorage[x].0.clone().into();
        jsonarray["table_schema"] = database.clone().into();
        jsonarray["columns"] = tablecoltypestorage[x].1.clone().into();
        jsonarray["types"] = tablecoltypestorage[x].2.clone().into();
        //grab first string in tuple
        let (constraitcol, constraintname) = tablecoltypestorage[x].3[0].clone();
        //put both into constraint column
        jsonarray["constraints"] = constraintname.clone().into();
        jsonarray["constraint_column"] = constraitcol.clone().into();
        

        jsondata[&x.to_string()] = jsonarray;
    }
    jsondata
}

//query constraints on table on database
//link to column name on table
pub fn query_constraints(
    table: &str,
    database: &str,
) -> std::result::Result<String, Box<dyn std::error::Error>> {
    let mut query = String::from("SELECT COLUMN_NAME, CONSTRAINT_NAME FROM INFORMATION_SCHEMA.KEY_COLUMN_USAGE WHERE TABLE_SCHEMA = '");
    query.push_str(database);
    query.push_str("' AND TABLE_NAME = '");
    query.push_str(table);
    query.push_str("'");
    Ok(query)
}
