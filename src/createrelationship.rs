use mysql::prelude::Queryable;

use crate::Reader;
use crate::dbconnect;

fn createrelationship(file:String)->String{
    //read file
    //create relationship
    //send to database

   //let conn= dbconnect::database_connection(database);
    let mut reader = Reader::from_path(file).unwrap();
    for result in reader.records() {
        let record = result.unwrap();
            let mut query:String= String::from("Alter Table ");
            query.push_str(&record[0]);
           // query.push_str(" Add Constraint ");
           // query.push_str(&record[0]);
           // query.push_str("_");
           // query.push_str(&record[1]);
           // query.push_str("_");
           // query.push_str(&record[2]);
           // query.push_str("_");
           // query.push_str(&record[3]);
            query.push_str(" Add Foreign Key (");
            query.push_str(&record[1]);
            query.push_str(") References ");
            query.push_str(&record[2]);
            query.push_str("(");
            query.push_str(&record[3]);
            query.push_str(")");
            query.push_str(" on Delete ");
            query.push_str(&record[4]);
            query.push_str(" on Update ");
            query.push_str(&record[5]);
            println!("{}", query);
            return query
    }
    String::from("Unable to create relationship")

}

pub fn commitrelationship(database: &str, file:String)->Result<Vec<String>, mysql::Error>{

//pub fn commitrelationship(database: &str,table1:&str, col1:&str, table2:&str,
//col2:&str,ondelete:&str, onupdate:&str)->Result<Vec<String>, mysql::Error>{
    let relation=createrelationship(file);
    //let relation=createrelationship_fromhtml(table1, col1, table2, col2, ondelete, onupdate);
    let mut conn= dbconnect::database_connection(database);
    
    let result:Vec<String>= conn.query(relation)?;
    return Ok(result)

}

pub fn commitrelationshipdefined(database: &str,table1:&str, col1:&str, table2:&str,
col2:&str,ondelete:&str, onupdate:&str)->Result<Vec<String>, mysql::Error>{
    let relation=createrelationship_fromhtml(table1, col1, table2, col2, ondelete, onupdate);
    println!("{}", relation);
    let mut conn= dbconnect::database_connection(database);
    
    let result:Vec<String>= conn.query(relation)?;
    return Ok(result)

}


fn createrelationship_fromhtml(table1:&str, col1:&str, table2:&str, col2:&str, ondelete:&str, onupdate:&str)->String{

   //let conn= dbconnect::database_connection(database);
    let mut query:String= String::from("Alter Table ");
    query.push_str(table1);
    query.push_str(" Add Foreign Key (");
    query.push_str(col1);
    query.push_str(") References ");
    query.push_str(table2);
    query.push_str("(");
    query.push_str(col2);
    query.push_str(")");
    query.push_str(" on Delete ");
    query.push_str(ondelete);
    query.push_str(" on Update ");
    query.push_str(onupdate);
    println!("{}", query);
    return query

}


