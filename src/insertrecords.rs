//accept json to insert record into database
//use serde to deserialize json to struct
//use mysql to insert record into database

use crate::pushdata::gettablecol;
use crate::dbconnect;
use crate::querytable;
use serde::{Deserialize, Serialize};
use serde_json::Result;
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
}
