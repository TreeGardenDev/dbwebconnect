use crate::Data2;
//use crate::Data;
use mysql::prelude::*;
use mysql::*;
pub mod gettablecol;
pub mod createtablestruct;


fn execute_insert2(
    data: Vec<Data2>,
    //data: &Vec<String>,
    tablename: &String,
    mut conn: PooledConn,
    database: String,
) -> std::result::Result<(), Box<dyn std::error::Error>> {
    
    let columname: Vec<String> = gettablecol::get_table_col(&mut conn, &tablename, &database).unwrap();
    println!("{:?}", columname);
    let insertstatement =gettablecol::createinsertstatement(&mut conn, &tablename, data, &database);
    println!("{}", insertstatement);

    let _=conn.query_drop(insertstatement);
    println!("Inserted data into table");
    Ok(())
}
