use mysql::{*, prelude::Queryable};
pub fn deleterecord(conn: &mut PooledConn, database:&str, table:&str,id:Vec<(String, String)>)-> Result<()>{
    //grab second string from tuple
    let mut stmt=String::from("DELETE FROM ");
    stmt.push_str(database);
    stmt.push_str(".");
    stmt.push_str(table);
    stmt.push_str(" WHERE ");
    stmt.push_str("INTERNAL_PRIMARY_KEY");
    stmt.push_str(" in( ");
    for i in 0..id.len(){
        stmt.push_str(&id[i].1);
        if i != id.len()-1{
            stmt.push_str(", ");
        }
    }
    stmt.push_str(")");
    println!("{}",stmt);
    conn.query_drop(stmt).unwrap();
    Ok(())

}
