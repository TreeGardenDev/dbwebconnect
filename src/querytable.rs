use mysql::prelude::*;
use mysql::*;
use crate::pushdata::gettablecol;
pub mod displayquery;
pub fn query_tables(table: &str, conn: &mut PooledConn, whereclause: &str, database: &str)->Vec<Vec<String>>{
    let columns = gettablecol::get_table_col(conn,table, database).unwrap();

    //let columntypes = grab_columntypes(conn, table, database).unwrap();

    let querydata = query_table(conn, table, whereclause, database, columns).unwrap();
    //columndata 
    querydata
    
}

fn grab_columntypes(conn: &mut PooledConn, table: &str, database: &str) -> std::result::Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut query = String::from("SELECT COLUMN_TYPE FROM INFORMATION_SCHEMA.COLUMNS WHERE TABLE_SCHEMA = '");
    query.push_str(database);
    query.push_str("' AND TABLE_NAME = '");
    query.push_str(table);
    query.push_str("'");
    let stmt: Vec<String> = conn.query_map(query, |datatype|datatype)?; //??
    Ok(stmt)
}

fn query_table(conn: &mut PooledConn, table: &str, whereclause: &str, database: &str, columntypes: Vec<String>) -> std::result::Result<Vec<Vec<String>>, Box<dyn std::error::Error>> {

//    let stmt: Vec<Vec<String>> = conn.query_map(query, |(col1)|{
//        let mut row: Vec<String> = Vec::new();
//        row.push(col1);
//    })?; //??

    let mut stmt=Vec::new();
   for i in 0..columntypes.len(){ 

       let mut query= String::from("SELECT ");
       query.push_str(&columntypes[i]);
       query.push_str(" FROM ");
       query.push_str(database);
       query.push_str(".");
       query.push_str(table);
       if whereclause != "" {
           query.push_str(" WHERE ");
           query.push_str(whereclause);
       }
       let row=conn.query_map(query.clone(), |columntypes:String|columntypes).unwrap();
        

       stmt.push(row); 
        query.clear();
    }
   //let mut fixedstmt:Vec<Vec<String>>=vec![&columntypes.len(), &stmt.len()];
   Ok(stmt) 
}
