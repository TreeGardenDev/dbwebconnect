//creates single csv record to be written to file
//
use csv::Writer;
use mysql::*;
use crate::pushdata::gettablecol;

pub fn create_session_csv(conn: &mut PooledConn, table: &str, database: &str) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let mut csv=Writer::from_path("data/sessioncsv/session.csv")?;
    let columns = gettablecol::get_table_col(conn,table, database).unwrap();
    for column in columns{
        csv.write_field(column);
    }
    //csv.write_record(&columns);
    csv.flush();
    Ok(())
}
