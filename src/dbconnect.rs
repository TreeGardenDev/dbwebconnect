//use crate::Reader;
use csv::ReaderBuilder;
use mysql::Pool;
use crate::PooledConn;
use crate::LinkDataBase;
pub fn database_connection(database: &str) -> PooledConn {
    let form = grabfromfile();
    let mut url=String::new();
    url.push_str("mysql://");
    url.push_str(&form.dbuser);
    url.push_str(":");
    url.push_str(&form.dbpass);
    url.push_str("@");
    url.push_str(&form.dbhost);
    url.push_str(":");
    url.push_str(&form.dbport);
    url.push_str("/");
    println!("{}", url);

    let url = format!("{}{}", url, database);
    let url = url.as_str();
    let pool = Pool::new(url).unwrap();
    let conn = pool.get_conn().unwrap();
    conn
}
pub fn database_connection_no_db_web(dbuser:&str, dbpassword:&str, dbport:&str, dbhost:&str) -> PooledConn {
    let mut url=String::new();
    url.push_str("mysql://");
    url.push_str(&dbuser);
    url.push_str(":");
    url.push_str(&dbpassword);
    url.push_str("@");
    url.push_str(&dbhost);
    url.push_str(":");
    url.push_str(&dbport);
    url.push_str("/");
    println!("{}", url);
    let url = url.as_str();
    let pool = Pool::new(url).unwrap();
    let conn = pool.get_conn().unwrap();
    return conn;

}
pub fn database_connection_no_db() -> PooledConn {
    let conn = internalqueryconn();
    return conn;

}
fn grabfromfile()->LinkDataBase{
    //igneroe header
    let mut reader=ReaderBuilder::new()
        .has_headers(false)
        .from_path("tmp/dbconnection.txt")
        .unwrap();
    let mut form = LinkDataBase{
        dbuser: String::new(),
        dbpass: String::new(),
        dbhost: String::new(),
        dbport: String::new(),
    };
    for result in reader.records() {
        //ignore header


        let record = result.unwrap();
        println!("{:?}", record);
        form.dbuser = record[0].to_string();
        form.dbpass = record[1].to_string();
        form.dbhost = record[2].to_string();
        form.dbport = record[3].to_string();
    }
    println!("grabfromfile");
    println!("{:?}", form);
    form
    
}

pub fn internalqueryconn()->PooledConn{
    let pword=std::env::args().nth(2).expect("no password given");
    //change
    //let url=String::from("mysql://root:secret@localhost:3306/");
    let mut url=String::from("mysql://root:");
    url.push_str(&pword);
    url.push_str("@localhost:3306/");
    let url = url.as_str();
    let pool = Pool::new(url).unwrap();
    let conn = pool.get_conn().unwrap();
    return conn;
}
pub fn internalqueryconnapikey()->PooledConn{
    //change
    let pword=std::env::args().nth(2).expect("no password given");
    //let url=String::from("mysql://root:secret@localhost:3306/ApiKey");
    let mut url=String::from("mysql://root:");
    url.push_str(&pword);
    url.push_str("@localhost:3306/ApiKey");
    let url = url.as_str();
    let pool = Pool::new(url).unwrap();
    let conn = pool.get_conn().unwrap();
    return conn;
}
