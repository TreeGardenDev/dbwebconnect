use crate::connkey;
use crate::connkey::ApiKey;
use crate::dbconnect;
use mysql::prelude::*;
//create a mysql database from a csv file
pub fn create_database(database_name: &str) {
    //let mut conn = mysql::Conn::new("mysql://kylelocal:kcb@127.0.0.1:3306/").unwrap();
    let mut conn = dbconnect::internalqueryconn();
    //let query = format!("CREATE DATABASE IF NOT EXISTS {}", database_name, ";");
    let mut key = ApiKey::new();
    key.populatekey(database_name.to_string());

    let hash = connkey::random_password();
    let mut query =
        ("CREATE DATABASE IF NOT EXISTS ".to_string() + database_name + ";").to_string();
    query.push_str("CREATE USER IF NOT EXISTS ");
    query.push_str(database_name);
    query.push_str("@'%' IDENTIFIED BY '");
    query.push_str(&hash);
    query.push_str("';");
    query.push_str("GRANT ALL PRIVILEGES ON ");
    query.push_str(database_name);
    query.push_str(".* TO ");
    query.push_str(database_name);
    query.push_str("@'%';");
    query.push_str("FLUSH PRIVILEGES;");

    conn.query_drop(query).unwrap();

    let _ = connkey::insert_apikey(database_name.to_string(), hash);
}

pub fn create_databaseweb(database: &str) {
    //let mut conn = mysql::Conn::new("mysql://kylelocal:kcb@127.0.0.1:3306/").unwrap();
    let mut conn = dbconnect::internalqueryconn();
    let dbname = String::from(database);
    //let mut conn=dbconnect::database_connection_no_db_web(database_user,database_password,database_host,port);
    let hash = connkey::random_password();
    //let query = format!("CREATE DATABASE IF NOT EXISTS {}", dbname);
    let mut query = ("CREATE DATABASE IF NOT EXISTS ".to_string() + &dbname + ";").to_string();
    query.push_str("CREATE USER IF NOT EXISTS ");
    query.push_str(&dbname);
    query.push_str("@'%' IDENTIFIED BY '");
    query.push_str(&hash);
    query.push_str("';");
    query.push_str("GRANT ALL PRIVILEGES ON ");
    query.push_str(&dbname);
    query.push_str(".* TO ");
    query.push_str(&dbname);
    query.push_str("@'%';");
    query.push_str("FLUSH PRIVILEGES;");

    conn.query_drop(query).unwrap();

    let _ = connkey::insert_apikey(dbname.to_string(), hash);
}
