use crate::LinkDataBase;
use crate::PooledConn;
use csv::ReaderBuilder;
use diesel::pg::PgConnection;
use diesel::prelude::*;

pub fn database_connection(_database: &str) -> PooledConn {
    internalqueryconn()
}
pub fn database_connection_no_db_web(
    _dbuser: &str,
    _dbpassword: &str,
    _dbport: &str,
    _dbhost: &str,
) -> PooledConn {
    internalqueryconn()
}
pub fn database_connection_no_db() -> PooledConn {
    internalqueryconn()
}
fn grabfromfile() -> LinkDataBase {
    let mut reader = ReaderBuilder::new()
        .has_headers(false)
        .from_path("tmp/dbconnection.txt")
        .unwrap();
    let mut form = LinkDataBase {
        dbuser: String::new(),
        dbpass: String::new(),
        dbhost: String::new(),
        dbport: String::new(),
    };
    for result in reader.records() {

        let record = result.unwrap();
        println!("{:?}", record);
        form.dbuser = record[0].to_string();
        form.dbpass = record[1].to_string();
        form.dbhost = record[2].to_string();
        form.dbport = record[3].to_string();
    }
    form
}

pub fn internalqueryconn() -> PooledConn {
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set for Postgres connection");
    PgConnection::establish(&database_url).expect("Failed to connect to Postgres")
}
pub fn internalqueryconnapikey() -> PooledConn {
    internalqueryconn()
}
