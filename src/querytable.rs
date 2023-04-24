use transpose::*;
use mysql::prelude::*;
use mysql::*;
use crate::pushdata::gettablecol;
pub mod displayquery;
pub fn query_tables(table: &str, conn: &mut PooledConn, whereclause: &str, database: &str)->Vec<Vec<String>>{
    let mut query= String::from("SELECT * FROM ");
    query.push_str(table);
    let columns = gettablecol::get_table_col(conn,table, database).unwrap();

    let columntypes = grab_columntypes(conn, table, database).unwrap();


    //let columndata=vec![columns,columntypes];
    //query table with columns in columns vector and type in columntypes vector

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

//    let stmt: Vec<Vec<String>> = conn.query_map(query, |(col1, col2, col3, col4, col5, col6, col7, col8, col9, col10)|{
//        let mut row: Vec<String> = Vec::new();
//        row.push(col1);
//        row.push(col2);
//        row.push(col3);
//        row.push(col4);
//        row.push(col5);
//        row.push(col6);
//        row.push(col7);
//        row.push(col8);
//        row.push(col9);
//        row.push(col10);
//        row
//    })?; //??
//
    let mut stmt=Vec::new();
   for i in 0..columntypes.len(){ 

       let mut query= String::from("SELECT ");
       query.push_str(&columntypes[i]);
       query.push_str(" FROM ");
       query.push_str(table);
       let mut row=(conn.query_map(query.clone(), |columntypes:String|columntypes).unwrap()); //??
        //transpose row vec
        //make row a column

       stmt.push(row); 
        query.clear();
    }
   //swap rows and columns stmt

   //transpose stmt
    //let stmt  = conn.query(query).unwrap();
    Ok(stmt)
}
