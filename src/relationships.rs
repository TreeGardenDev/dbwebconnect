use serde::{Deserialize, Serialize};
use serde_json::Result;
use mysql::{*, prelude::Queryable};

#[derive(Debug, Clone, Deserialize)]
pub struct Relationship_Builder{
    pub database: String,
    pub parent_table: String,
    pub child_table: String,
    pub where_clause: String,
    pub relationship_name: String,
}
impl Relationship_Builder{
    pub fn new(database:&str, parent_table:&str, child_table:&str, relationship_name:&str, where_clause:&str)->Relationship_Builder {
        println!("Building Relationship");
        println!("Database: {}", database);
        println!("Parent Table: {}", parent_table);
        println!("Child Table: {}", child_table);
        println!("Relationship Name: {}", relationship_name);
        println!("Where Clause: {}", where_clause);

        let relationship = Relationship_Builder{
            database: database.to_string(),
            parent_table: parent_table.to_string(),
            child_table: child_table.to_string(),
            where_clause: where_clause.to_string(),
            relationship_name: relationship_name.to_string()
        };
        relationship
    }
    pub fn check_relationship_name(&self, conn: &mut PooledConn)->bool{
        let stmt = format!("SELECT relationship FROM Relationships.relationships WHERE relationship='{}'", self.relationship_name);
        let result:Vec<String> = conn.query_map(stmt, |relationship| relationship).unwrap();
        if result.len() > 0{
            println!("Relationship Name Already Exists");
            return false;
        }
        else{
            println!("Relationship Name Does Not Exist");
            return true;
        }
    }
        
    
}

pub fn create_relationship_stmt(relationship: &Relationship_Builder) -> String{
    let stmt = format!("INSERT INTO Relationships.relationships (targeted_database, parent_table, child_table, where_clause, relationship) VALUES ('{}', '{}', '{}', '{}', '{}')", relationship.database, relationship.parent_table, relationship.child_table, relationship.where_clause, relationship.relationship_name);
    stmt
}
pub fn execute_relationship_stmt(stmt: &str, conn: &mut PooledConn){
    let _=conn.query_drop(stmt);
}
