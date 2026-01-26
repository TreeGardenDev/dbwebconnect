use crate::Data2;
use crate::PooledConn;
use diesel::prelude::*;
use diesel::sql_query;
use diesel::sql_types::Text;
use diesel::QueryableByName;
pub fn get_table_col(
    conn: &mut PooledConn,
    table_name: &str,
    database_name: &str,
) -> std::result::Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut querystring: String =
	String::from("SELECT COLUMN_NAME FROM INFORMATION_SCHEMA.COLUMNS WHERE TABLE_SCHEMA='");
    querystring.push_str(database_name.to_string().as_str());
    querystring.push_str("' AND TABLE_NAME='");
    //testcsv' AND TABLE_NAME='");
    querystring.push_str(table_name.to_string().as_str());
    querystring.push_str("'");
    // In Postgres, unquoted identifiers are stored lowercased in information_schema,
    // so filter on the lowercased names of our internal/system columns.
    querystring.push_str(" and COLUMN_NAME != 'internal_primary_key'");
    querystring.push_str(" and COLUMN_NAME != 'gps_id'");
    querystring.push_str(" and COLUMN_NAME != 'x_coord'");
    querystring.push_str(" and COLUMN_NAME != 'y_coord'");
    querystring.push_str(" and COLUMN_NAME != 'attachment'");
    #[derive(QueryableByName)]
    struct ColName {
        #[diesel(sql_type = Text)]
        column_name: String,
    }

    let rows: Vec<ColName> = sql_query(querystring).load(conn)?;
    Ok(rows.into_iter().map(|r| r.column_name).collect())
}

pub fn createinsertstatement(
    conn: &mut PooledConn,
    table_name: &str,
    data: Vec<Data2>,
    database: &str,
) -> String {
    let mut insertstatement = String::from("insert into ");
    insertstatement.push_str(database);
    insertstatement.push_str(".");
    insertstatement.push_str(table_name);
    insertstatement.push_str(" (");
    let col_vec = get_table_col(conn, table_name, database).unwrap();
    for col in &col_vec {
        insertstatement.push_str(&col);
        insertstatement.push_str(",");
    }
    insertstatement.pop();
    insertstatement.push_str(") values (");
    for i in 0..data.len() {
        for j in 0..data[i].columns.len() {
            println!("New Column");
            for k in 0..data[i].columns[j].len() {
                //println!("Data below");
                println!("{:?}", data[i].columns[j][k]);
                //println!("Data above");
                let datarecord = &data[i].columns[j][k];
                //insert into mysql data from data variable into columns in columnname variable
                //let insertstatement =gettablecol::createinsertstatement(&mut conn, &tablename);
                //println!("{}", insertstatement);
                insertstatement.push_str("'");
                insertstatement.push_str(&datarecord);
                insertstatement.push_str("'");
                insertstatement.push_str(",");
            }
            insertstatement.pop();
            insertstatement.push_str("),(");
        }
        insertstatement.pop();
    }
    insertstatement.pop();
    insertstatement.push_str(";");
    insertstatement
}
