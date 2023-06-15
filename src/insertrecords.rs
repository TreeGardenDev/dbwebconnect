//accept json to insert record into database
//use serde to deserialize json to struct
//use mysql to insert record into database

use crate::pushdata::gettablecol;
use crate::dbconnect;
use crate::querytable;
use mysql::prelude::Queryable;
use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};
use mysql;

#[derive(Serialize, Deserialize, Debug)]
pub struct TableDef {
    pub table_name: String,
    pub table_fields: Vec<String>,
    pub table_types: Vec<String>,
}
impl TableDef {
    pub fn new() -> TableDef {
        TableDef {
            table_name: String::new(),
            table_fields: Vec::new(),
            table_types: Vec::new(),
        }
    }
    pub fn populate(&mut self, table_name:&str, database: &str){
        let mut conn = dbconnect::internalqueryconn();


        let types= querytable::grab_columntypes(&mut conn,table_name, database).unwrap();
        let fields= gettablecol::get_table_col(&mut conn,table_name, database).unwrap();
        self.table_name=String::from(table_name);
        self.table_fields=fields;
        self.table_types=types;


        
    }
    pub fn compare_fields(&mut self, date:&Vec<(String,String)>)->bool{
        let mut matched:Vec<bool>=Vec::new();
        if self.table_fields.len() != date.len(){
            return false;
        }
        for i in 0..date.len(){
            matched.push(false);
            for j in 0..self.table_fields.len(){
                if self.table_fields[j] == date[i].0{
                    matched[i]=true;
                }

            }
        }
        for i in 0..matched.len(){
            if matched[i]==false{
                return false;
            }
        }
        true
    }
    pub fn insert(self, data:&Vec<(String, String)>, table: &str, database: &str)->String{
        let mut stmt= String::from("INSERT INTO ");
        stmt.push_str(database);
        stmt.push_str(".");
        stmt.push_str(table);
        stmt.push_str(" (");
        for i in 0..data.len(){
            stmt.push_str(&data[i].0);
            if i != data.len()-1{
                stmt.push_str(", ");
            }
        }
        stmt.push_str(") VALUES (");
        for i in 0..data.len(){
            let typestring=&self.compare_types(&data[i].0, &self.table_fields, &self.table_types);

            match typestring.as_str(){
                "int(11)" => {
                    stmt.push_str(&data[i].1);

                }
                "varchar(255)" => {
                    stmt.push_str("'");
                    stmt.push_str(&data[i].1);
                    stmt.push_str("'");
                }
                "int(100)" => {
                    stmt.push_str(&data[i].1);
                }
                "varchar(100)" => {
                    stmt.push_str("'");
                    stmt.push_str(&data[i].1);
                    stmt.push_str("'");
                }
                _ => {
                    stmt.push_str("NULL");
                }
            }

            if i != data.len()-1{
                stmt.push_str(", ");
            }
        }
        stmt.push_str(")");
        stmt

    }
    fn compare_types(&self, column: &str, fields: &Vec<String>, types: &Vec<String>)->String{
        let colstring=String::from(column);
        for i in 0..fields.len(){
            if fields[i]==colstring{
                return types[i].clone();
                
            }
        }
        String::from("NULL")

    }
}
pub fn exec_insert(statement:String)->Result<String>{
    let mut conn = dbconnect::internalqueryconn();
    conn.query_drop(statement).unwrap();
    Ok(String::from("Success"))
}


