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

pub fn parse_json(json: String)->String{
    let json=json.replace("{", "");
    let json=json.replace("}", "");
    let json=json.replace("\"", "");
    let json=json.replace(":", "");
    let json=json.replace(",", "");
    let json=json.replace("[", "");
    let json=json.replace("]", "");
    let json=json.replace(" ", "");
    let json=json.replace("\n", "");
    let json=json.replace("\t", "");
    let json=json.replace("\r", "");
    let json=json.replace('"', "");

    json

}
pub fn createrelationshipfromweb(database: &str, json:Vec<(String,String)>)->String{

    //grab second string where first string is equal to table1
    let mut table1=String::from("");
    let mut col1=String::from("");
    let mut table2=String::from("");
    let mut col2=String::from("");
    let mut ondelete=String::from("");
    let mut onupdate=String::from("");
    for (key, value) in json {
        if key=="table1"{
            table1=value;
        }
        else if key=="table1col"{
            col1=value;
        }
        else if key=="table2"{
            table2=value;
        }
        else if key=="table2col"{
            col2=value;
        }
        else if key=="ondelete"{
            ondelete=value;
        }
        else if key=="onupdate"{
            onupdate=value;
        }
    }


    let mut query:String= String::from("Alter Table ");
    query.push_str(&database);
    query.push_str(".");
    query.push_str(&table1);
    query.push_str(" Add Foreign Key (");
    query.push_str(&col1);
    query.push_str(") References ");
    query.push_str(&database);
    query.push_str(".");
    query.push_str(&table2);
    query.push_str("(");
    query.push_str(&col2);
    query.push_str(")");
    query.push_str(" on Delete ");
    query.push_str(&ondelete);
    query.push_str(" on Update ");
    query.push_str(&onupdate);
    println!("{}", query);
    return query


}
pub fn commitrelationshipfromweb(statement:String)->Result<String, mysql::Error>{
    let mut conn= dbconnect::internalqueryconn();
    conn.query_drop(statement)?;
    
    return Ok(String::from("Success"))
}
