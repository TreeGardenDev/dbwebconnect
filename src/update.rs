use diesel::prelude::*;
use diesel::sql_query;
use crate::PooledConn;
pub fn updaterecord(database: &str, table: &str, date: Vec<Vec<(String, String)>>) -> Vec<String> {
    let mut stmts = Vec::new();
    for data in date.iter() {
        let mut stmt = String::from("UPDATE ");
        stmt.push_str(database);
        stmt.push_str(".");
        stmt.push_str(table);
        stmt.push_str(" SET ");
        // Find the primary key entry (INTERNAL_PRIMARY_KEY), case-insensitive.
        let pk_index_opt = data
            .iter()
            .position(|(k, _)| k.eq_ignore_ascii_case("INTERNAL_PRIMARY_KEY"));

        let pk_index = match pk_index_opt {
            Some(idx) => idx,
            None => {
                // No primary key provided; skip this record.
                continue;
            }
        };

        // Build SET clause for all non-PK fields.
        let mut first = true;
        for (i, (col, val)) in data.iter().enumerate() {
            if i == pk_index {
                continue;
            }
            if !first {
                stmt.push_str(", ");
            }
            first = false;

            // Clean and quote value as a SQL string literal.
            let mut valuedata = val.replace("\"", "");
            valuedata = valuedata.replace("'", "''");

            stmt.push_str(col);
            stmt.push_str(" = '");
            stmt.push_str(&valuedata);
            stmt.push_str("'");
        }

        // WHERE clause on primary key value.
        let mut pk_value = data[pk_index].1.replace("\"", "");
        pk_value = pk_value.replace("'", "''");

        stmt.push_str(" WHERE INTERNAL_PRIMARY_KEY = ");
        stmt.push_str(&pk_value);

        stmts.push(stmt);
    }
    println!("{:?}", stmts);

    stmts
}
pub fn executeupdaterecord(
    conn: &mut PooledConn,
    statement: &str,
) -> std::result::Result<String, String> {
    sql_query(statement)
        .execute(conn)
        .expect("Failed to execute UPDATE statement");
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
