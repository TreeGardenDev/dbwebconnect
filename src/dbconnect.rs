use mysql::Pool;
use crate::PooledConn;
pub fn database_connection(database: &str) -> PooledConn {
    let url = "mysql://kylelocal:kcb@127.0.0.1:3306/";
    //grab user and password from AppData actix-web
    




    //add database name to url
    let url = format!("{}{}", url, database);
    //make url usable by pool
    let url = url.as_str();
    let pool = Pool::new(url).unwrap();
    let conn = pool.get_conn().unwrap();
    conn
}
