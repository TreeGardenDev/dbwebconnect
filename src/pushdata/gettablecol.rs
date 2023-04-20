use mysql::prelude::*;
use crate::Data2;
use mysql::*;
pub fn get_table_col(conn: &mut PooledConn, table_name: &str, database_name: &str)
 -> std::result::Result<Vec<String>, Box<dyn std::error::Error>> {
   let mut querystring:String=String::from("SELECT COLUMN_NAME FROM INFORMATION_SCHEMA.COLUMNS WHERE TABLE_SCHEMA='");
   querystring.push_str(database_name.to_string().as_str());
   querystring.push_str("' AND TABLE_NAME='");
       //testcsv' AND TABLE_NAME='");
   querystring.push_str(table_name.to_string().as_str());
   querystring.push_str("'");
    //let columnname = conn.query_map("SELECT COLUMN_NAME FROM INFORMATION_SCHEMA.COLUMNS WHERE TABLE_SCHEMA='testcsv' AND TABLE_NAME='Data'", |(COLUMN_NAME)| COLUMN_NAME)?;
    let columnname = conn.query_map(querystring, |(column_name)| column_name)?;

    Ok((columnname))

}


pub fn createinsertstatement(conn: &mut PooledConn, table_name: &str, data:Vec<Data2>, database: &str) -> String
{
    let mut insertstatement = String::from("insert into ");
    insertstatement.push_str(table_name);
    insertstatement.push_str(" (");
    let mut col_vec = get_table_col(conn, table_name, database).unwrap();
    for col in &col_vec {
        insertstatement.push_str(&col);
        insertstatement.push_str(",");
    }
    insertstatement.pop();
   insertstatement.push_str(") values (");
    for i in 0..data.len(){
        for j in 0..data[i].columns.len(){
            println!("New Column");
            for k in 0..data[i].columns[j].len(){
                //println!("Data below");
                println!("{:?}", data[i].columns[j][k]);
                //println!("Data above");
                let datarecord=&data[i].columns[j][k];
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
