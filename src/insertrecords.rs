use crate::dbconnect;
use crate::pushdata::gettablecol;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use diesel::prelude::*;
use diesel::sql_query;

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
    pub fn populate(&mut self, table_name: &str, database: &str) {
        let mut conn = dbconnect::internalqueryconn();

        // For Postgres we no longer depend on MySQL's INFORMATION_SCHEMA.COLUMN_TYPE;
        // we only need the column names and will send values as string literals,
        // letting Postgres cast as needed.
        let fields = gettablecol::get_table_col(&mut conn, table_name, database).unwrap();
        println!("Fields: {:?}", fields);
        self.table_name = String::from(table_name);
        self.table_fields = fields;
        self.table_types = Vec::new();
    }
    pub fn compare_fields(&mut self, data: &Vec<Vec<(String, String)>>) -> bool {
        //default to false

        for date in data.iter() {
            let mut matched: Vec<bool> = Vec::new();
            if self.table_fields.len() != date.len() {
                return false;
            }
            for i in 0..date.len() {
                matched.push(false);
                for j in 0..self.table_fields.len() {
                    if self.table_fields[j] == date[i].0 {
                        matched[i] = true;
                    }
                }
            }
            for i in 0..matched.len() {
                if matched[i] == false {
                    return false;
                }
            }
            println!("{:?}", matched);
        }

        true
    }
    pub fn insert(self, date: &Vec<Vec<(String, String)>>, table: &str, database: &str) -> String {
        let mut stmt = String::from("INSERT INTO ");
        stmt.push_str(database);
        stmt.push_str(".");
        stmt.push_str(table);
        stmt.push_str(" (");
        let data1 = &date[0];
        for i in 0..data1.len() {
            stmt.push_str(&data1[i].0);
            if i != data1.len() - 1 {
                stmt.push_str(", ");
            }
        }
        stmt.push_str(") VALUES (");
        for data in date.iter() {
            for i in 0..data.len() {
                let mut valuedata = data[i].1.replace("\"", "");
                // Escape single quotes for SQL literal
                valuedata = valuedata.replace("'", "''");

                // For Postgres we send everything as a quoted string literal and
                // let the engine cast to the target column type.
                stmt.push_str("'");
                stmt.push_str(&valuedata);
                stmt.push_str("'");

                if i != data.len() - 1 {
                    stmt.push_str(", ");
                }
            }
            stmt.push_str("), (");
        }
        // remove the trailing `, (`
        stmt.pop();
        stmt.pop();
        stmt.pop();
        stmt
    }
}
pub fn insert_attachment(
    database: &str,
    table: &str,
    filename: &str,
    data: Vec<u8>,
) -> Result<String> {
    let mut stmt = String::from("INSERT INTO ");
    stmt.push_str(database);
    stmt.push_str(".");
    stmt.push_str(table);
    stmt.push_str("_");
    stmt.push_str("GPS");
    stmt.push_str(" (X_COORD, ATTACHMENT) VALUES ('");
    stmt.push_str(filename);
    stmt.push_str("', '");
    stmt.push_str(
        data.iter()
            .map(|x| format!("{:02X}", x))
            .collect::<String>()
            .as_str(),
    );
    // stmt.push_str(&String::from_utf8(data).unwrap());
    stmt.push_str("')");
    Ok(stmt)
}
pub fn exec_insert(statement: String) -> Result<String> {
    let mut conn = dbconnect::internalqueryconn();
    sql_query(statement)
        .execute(&mut conn)
        .expect("Insert failed");
    Ok(String::from("Success"))
}
