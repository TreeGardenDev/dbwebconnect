use crate::Reader;
use mysql::Pool;
use crate::PooledConn;
use crate::LinkDataBase;
pub fn database_connection(database: &str) -> PooledConn {
    let url = "mysql://kylelocal:kcb@127.0.0.1:3306/";
    //grab user and password from AppData actix-web
    grabfromfile();



    //add database name to url
    let url = format!("{}{}", url, database);
    //make url usable by pool
    let url = url.as_str();
    let pool = Pool::new(url).unwrap();
    let conn = pool.get_conn().unwrap();
    conn
}

fn grabfromfile()->LinkDataBase{
    let mut reader = Reader::from_path("tmp/dbconnection.txt").unwrap();
    let mut form = LinkDataBase{
        dbuser: String::new(),
        dbpass: String::new(),
        dbhost: String::new(),
        dbport: String::new(),
    };
    for result in reader.records() {
        let record = result.unwrap();
        form.dbuser = record[0].to_string();
        form.dbpass = record[1].to_string();
        form.dbhost = record[2].to_string();
        form.dbport = record[3].to_string();
    }
    println!("grabfromfile");
    println!("{:?}", form);
    form
    
}
