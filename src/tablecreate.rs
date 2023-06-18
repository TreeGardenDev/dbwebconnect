use mysql::prelude::*;
use mysql::*;
//read from csv file to create table in mariadb with given column names
pub fn create_table(
    conn: &mut PooledConn,
    database: &str,
    table_name: &str,
    column_names: &Vec<String>,
    column_types: &Vec<String>,
) {
    let mut query = String::from("CREATE TABLE ");
    query.push_str(database);
    query.push_str(".");
    query.push_str(table_name);
    query.push_str(" (");
    for i in 0..column_names.len() {
        query.push_str(column_names[i].as_str());
        query.push_str(" ");
        query.push_str(column_types[i].as_str());
        query.push_str(", ");
    }
    query.pop();
    query.pop();
    query.push_str(")");
    println!("{}", query);
    conn.query_drop(query).unwrap();
}
pub fn create_table_web(
    conn: &mut PooledConn,
    database: &str,
    table_name: &str,
    column_names: &Vec<(String, String)>,
    column_types: &Vec<(String, String)>,
) {
    let mut query = String::from("CREATE TABLE ");
    query.push_str(database);
    query.push_str(".");
    query.push_str(table_name);
    query.push_str(" (");
    query.push_str("INTERNAL_PRIMARY_KEY INT NOT NULL AUTO_INCREMENT PRIMARY KEY, ");
    for i in 0..column_names.len() {
        query.push_str(column_names[i].1.as_str());
        query.push_str(" ");
        query.push_str(column_types[i].1.as_str());
        query.push_str(", ");
    }
    query.pop();
    query.pop();
    query.push_str(")");
    println!("{}", query);
    conn.query_drop(query).unwrap();
}

pub fn parse_json(json: Vec<(String, String)>) -> (Vec<(String, String)>, Vec<(String, String)>) {
    let mut columnstr = json[0].1.clone();
    let mut datatypestr = json[1].1.clone();
    columnstr = columnstr.replace("\"", "");
    datatypestr = datatypestr.replace("\"", "");
    columnstr = columnstr.replace("[", "");
    datatypestr = datatypestr.replace("[", "");
    columnstr = columnstr.replace("]", "");
    datatypestr = datatypestr.replace("]", "");
    columnstr = columnstr.replace(" ", "");
    datatypestr = datatypestr.replace(" ", "");
    columnstr = columnstr.replace("\n", "");
    datatypestr = datatypestr.replace("\n", "");
    columnstr = columnstr.replace("\r", "");
    datatypestr = datatypestr.replace("\r", "");
    columnstr = columnstr.replace("\t", "");
    datatypestr = datatypestr.replace("\t", "");
    columnstr = columnstr.replace("\r\n", "");
    datatypestr = datatypestr.replace("\r\n", "");
    columnstr = columnstr.replace("{", "");
    datatypestr = datatypestr.replace("{", "");
    columnstr = columnstr.replace("}", "");
    datatypestr = datatypestr.replace("}", "");

    let column = columnstr.split(",");
    let datatype = datatypestr.split(",");
    let column: Vec<String> = column.map(|s| s.to_string()).collect();
    let datatype: Vec<String> = datatype.map(|s| s.to_string()).collect();
    //split string in vector by colon
    //push into vector
    let mut splitcolumn: Vec<(String, String)> = Vec::new();
    let mut splitdatatype: Vec<(String, String)> = Vec::new();
    for i in 0..column.len() {
        let split = column[i].split(":");
        let split2 = datatype[i].split(":");
        let splitvec: Vec<&str> = split.collect();
        let splitvec2: Vec<&str> = split2.collect();
        splitcolumn.push((splitvec[0].to_string(), splitvec[1].to_string()));
        splitdatatype.push((splitvec2[0].to_string(), splitvec2[1].to_string()));
    }

    (splitcolumn, splitdatatype)
}
