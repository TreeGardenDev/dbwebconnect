use csv::StringRecord; use crate::Data2;
//use crate::Data;
use mysql::prelude::*;
use mysql::*;
use crate::Reader;
use crate::Table;
pub mod gettablecol;
pub mod createtablestruct;
#[derive(Debug)] struct InsertData<'a>{
    data: Vec<&'a str>,
}


fn execute_insert2(
    data: Vec<Data2>,
    //data: &Vec<String>,
    tablename: String,
    mut conn: PooledConn,
    columnames: Vec<&str>,
    database: String,
) -> std::result::Result<(), Box<dyn std::error::Error>> {
    
    let columname: Vec<String> = gettablecol::get_table_col(&mut conn, &tablename, &database).unwrap();
    println!("{:?}", columname);
    let insertstatement =gettablecol::createinsertstatement(&mut conn, &tablename, data, &database);
    println!("{}", insertstatement);

    conn.query_drop(insertstatement)?;
    Ok(())
}
