use crate::Reader;
use csv::ReaderBuilder;
use mysql::Pool;
use crate::PooledConn;
use crate::LinkDataBase;
pub fn database_connection(database: &str) -> PooledConn {
    let form = grabfromfile();
    //let url = "mysql://kylelocal:kcb@127.0.0.1:3306/";
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
    //grab user and password from AppData actix-web



    //add database name to url
    let url = format!("{}{}", url, database);
    //make url usable by pool
    let url = url.as_str();
    let pool = Pool::new(url).unwrap();
    let conn = pool.get_conn().unwrap();
    conn
}
pub fn database_connection_no_db() -> PooledConn {
    let form = grabfromfile();
    //let url = "mysql://kylelocal:kcb@localhost:3306/";
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
    let url = url.as_str();
    let pool = Pool::new(url).unwrap();
    let conn = pool.get_conn().unwrap();
    return conn;

}
fn grabfromfile()->LinkDataBase{
    //let mut reader = Reader::from_path("tmp/dbconnection.txt").unwrap();
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
