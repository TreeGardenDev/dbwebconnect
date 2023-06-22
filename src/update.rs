use mysql::{prelude::Queryable, *};
pub fn updaterecord(database: &str, table: &str, date: Vec<Vec<(String, String)>>) -> Vec<String> {
    let mut stmts=Vec::new();
    for data in date.iter(){

    let mut stmt = String::from("UPDATE ");
    stmt.push_str(database);
    stmt.push_str(".");
    stmt.push_str(table);
    stmt.push_str(" SET ");
        for i in 1..data.len() {
            stmt.push_str(&data[i].0);
            stmt.push_str("= \"");

            stmt.push_str(&data[i].1);
            stmt.push_str("\"");
            if i != data.len() - 1 {
                stmt.push_str(", ");
            }
        }
    
        stmt.push_str(" WHERE ");
        stmt.push_str("INTERNAL_PRIMARY_KEY");
        stmt.push_str("=");
        stmt.push_str(&data[0].1);
    
    stmts.push(stmt);
    }
    println!("{:?}", stmts);

    stmts
}
pub fn executeupdaterecord(conn: &mut PooledConn, statement: &str) -> Result<String> {
    conn.query_drop(statement).unwrap();
    Ok(String::from("Success"))
}

//#[cfg(test)]
//mod tests {
//    use super::*;
//    #[test]
//    fn test_updaterecord(){
//        let mut conn=connect("mysql://root:password@localhost:3306/").unwrap();
//        let data=vec![("INTERNAL_PRIMARY_KEY".to_string(),"1".to_string()),("1".to_string(),"'test'".to_string())];
//        updaterecord(conn,"test","test",&data).unwrap();
//
//    }
//}
