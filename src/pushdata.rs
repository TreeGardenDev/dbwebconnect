use crate::Data2;
use crate::PooledConn;
//use crate::Data;
use diesel::prelude::*;


pub mod createtablestruct;
pub mod gettablecol;

fn execute_insert2(
    data: Vec<Data2>,
    //data: &Vec<String>,
    tablename: &String,
    mut conn: PooledConn,
    database: String,
) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let columname: Vec<String> =
        gettablecol::get_table_col(&mut conn, &tablename, &database).unwrap();
    println!("{:?}", columname);
    let insertstatement =
        gettablecol::createinsertstatement(&mut conn, &tablename, data, &database);
    println!("{}", insertstatement);
    diesel::sql_query(insertstatement)
        .execute(&mut conn)
        .expect("Bulk insert failed");
    println!("Inserted data into table");
    Ok(())
}
