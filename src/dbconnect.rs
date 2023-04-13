use mysql::Pool;
use crate::PooledConn;
pub fn database_connection() -> PooledConn {
    let url = "mysql://kylelocal:kcb@127.0.0.1:3306/testcsv";
    let pool = Pool::new(url).unwrap();
    let mut conn = pool.get_conn().unwrap();
    conn
}
