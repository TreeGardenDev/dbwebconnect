//creates single csv record to be written to file
//
use csv::Writer;
use mysql::*;
use crate::pushdata::gettablecol;
pub mod generateform;
pub fn create_session_csv(conn: &mut PooledConn, table: &str, database: &str) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let mut file=String::from("data/sessioncsv/");
    file.push_str(table);
    file.push_str(".csv");
    let mut csv=Writer::from_path(file)?;
    let columns = gettablecol::get_table_col(conn,table, database).unwrap();
    for column in columns{
        csv.write_field(column)?;
    }
    //csv.write_record(&columns);
    csv.flush()?;
    Ok(())
}
pub fn create_row_csv(conn: &mut PooledConn, table: &str, database: &str) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let mut file=String::from("data/sessioncsv/");
    file.push_str(table);
    file.push_str(".csv");
    //check if file exists
    if std::path::Path::new(&file).exists(){
        //file exists
        let mut csv=Writer::from_path(&file)?;
        let columns = gettablecol::get_table_col(conn,table, database).unwrap();

        for column in columns{
            csv.write_field(column)?;
        }
        //csv.write_record(&columns);
        csv.flush()?;
    }else{
        create_session_csv(conn, table, database)?;
        create_row_csv(conn, table, database)?;   
    }
    Ok(())
}
