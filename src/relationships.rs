use crate::PooledConn;
use diesel::prelude::*;
use diesel::sql_query;
use serde::{Deserialize, Serialize};

#[derive(Serialize,Debug, Clone, Deserialize)]
pub struct RelationshipBuilder{
    pub database: String,
    pub parent_table: String,
    pub child_table: String,
    pub where_clause: String,
    pub relationship_name: String,
}
impl RelationshipBuilder{
    pub fn new(database:&str, parent_table:&str, child_table:&str, relationship_name:&str, where_clause:&str)->RelationshipBuilder {
        println!("Building Relationship");
        println!("Database: {}", database);
        println!("Parent Table: {}", parent_table);
        println!("Child Table: {}", child_table);
        println!("Relationship Name: {}", relationship_name);
        println!("Where Clause: {}", where_clause);

        let relationship = RelationshipBuilder{
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

        #[derive(QueryableByName)]
        struct RelRow {
            #[diesel(sql_type = diesel::sql_types::Text)]
            relationship: String,
        }

        let result: Vec<RelRow> = sql_query(stmt)
            .load(conn)
            .expect("Failed to query relationships");
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

pub fn create_relationship_stmt(relationship: &RelationshipBuilder) -> String{
    let stmt = format!("INSERT INTO Relationships.relationships (targeted_database, parent_table, child_table, where_clause, relationship) VALUES ('{}', '{}', '{}', '{}', '{}')", relationship.database, relationship.parent_table, relationship.child_table, relationship.where_clause, relationship.relationship_name);
    stmt
}
pub fn execute_relationship_stmt(stmt: &str, conn: &mut PooledConn){
    let _ = sql_query(stmt)
        .execute(conn)
        .expect("Failed to execute relationship insert");
}
pub fn query_relationships(conn: &mut PooledConn, relationship_name:&str)->Vec<RelationshipBuilder>{
    let mut stmt = String::from("SELECT targeted_database, parent_table, child_table, where_clause, relationship FROM Relationships.relationships");
    stmt.push_str(" WHERE relationship='");
    stmt.push_str(relationship_name);
    stmt.push_str("'");

    #[derive(QueryableByName)]
    struct RelFullRow {
        #[diesel(sql_type = diesel::sql_types::Text)]
        targeted_database: String,
        #[diesel(sql_type = diesel::sql_types::Text)]
        parent_table: String,
        #[diesel(sql_type = diesel::sql_types::Text)]
        child_table: String,
        #[diesel(sql_type = diesel::sql_types::Text)]
        where_clause: String,
        #[diesel(sql_type = diesel::sql_types::Text)]
        relationship: String,
    }

    let rows: Vec<RelFullRow> = sql_query(stmt)
        .load(conn)
        .expect("Failed to query relationship rows");

    rows
        .into_iter()
        .map(|r| RelationshipBuilder {
            database: r.targeted_database,
            parent_table: r.parent_table,
            child_table: r.child_table,
            where_clause: r.where_clause,
            relationship_name: r.relationship,
        })
        .collect()
}
