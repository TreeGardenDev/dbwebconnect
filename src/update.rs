use mysql::{*, prelude::Queryable};
pub fn updaterecord(mut conn: PooledConn, database:&str, table:&str, data:Vec<(String,String)>)-> Result<()>{
    let mut stmt=String::from("UPDATE ");
    stmt.push_str(database);
    stmt.push_str(".");
    stmt.push_str(table);
    stmt.push_str(" SET ");
    for i in 1..data.len(){
        stmt.push_str(&data[i].0);
        stmt.push_str("=");
        stmt.push_str(&data[i].1);
        if i != data.len()-1{
            stmt.push_str(", ");
        }
    }
    stmt.push_str(" WHERE ");
    stmt.push_str("INTERNAL_PRIMARY_KEY");
    stmt.push_str("=");
    stmt.push_str(&data[0].1);
    println!("{}",stmt);

    conn.query_drop(stmt).unwrap();
    Ok(())

}
