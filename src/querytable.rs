use mysql::prelude::*;
use mysql::*;
use serde_json::json;

//use crate::{connkey, dbconnect};
pub mod displayquery;
pub fn query_tables(
    table: &str,
    conn: &mut PooledConn,
    whereclause: &str,
    database: &str,
    select: Vec<&str>,
) -> Vec<Vec<String>> {
    
    let columns_stmt = grab_columnnames(table, database, select).unwrap();
    let columns = exec_map(conn, &columns_stmt).unwrap();


    let querydata = query_table(conn, table, whereclause, database, columns).unwrap();

    querydata
}

pub fn exec_map(
    conn: &mut PooledConn,
    query: &str,
) -> std::result::Result<Vec<String>, Box<dyn std::error::Error>> {

    let stmt: Vec<String> = conn.query_map_opt(query, |data | data.unwrap_or("".to_string()))?;

    //

    Ok(stmt)
}
pub fn exec_map_tuple(
    conn: &mut PooledConn,
    query: &str,
) -> std::result::Result<Vec<(String,String)>, Box<dyn std::error::Error>> {
    let stmt: Vec<(String,String)> = conn.query_map(query, |(data1,data2)| (data1,data2))?;
    Ok(stmt)
}
pub fn grab_columntypes(
    table: &str,
    database: &str,
) -> std::result::Result<String, Box<dyn std::error::Error>> {
    let mut query =
        String::from("SELECT COLUMN_TYPE FROM INFORMATION_SCHEMA.COLUMNS WHERE TABLE_SCHEMA = '");
    query.push_str(database);
    query.push_str("' AND TABLE_NAME = '");
    query.push_str(table);
    query.push_str("'");
    query.push_str("And COLUMN_NAME != 'INTERNAL_PRIMARY_KEY'");
    query.push_str("And COLUMN_NAME != 'GPS_ID'");
    query.push_str("And COLUMN_NAME != 'X_COORD'");
    query.push_str("And COLUMN_NAME != 'Y_COORD'");
    query.push_str("And COLUMN_NAME != 'Attachment'");

    Ok(query)
}

pub fn grab_all_columntypes(
    table: &str,
    database: &str,
) -> std::result::Result<String, Box<dyn std::error::Error>> {
    let mut query =
        String::from("SELECT COLUMN_TYPE FROM INFORMATION_SCHEMA.COLUMNS WHERE TABLE_SCHEMA = '");
    query.push_str(database);
    query.push_str("' AND TABLE_NAME = '");
    query.push_str(table);
    query.push_str("'");

    Ok(query)
}
pub fn grab_columntypes_schema(
    table: &str,
    database: &str,
) -> std::result::Result<String, Box<dyn std::error::Error>> {
    let mut query =
        String::from("SELECT COLUMN_TYPE FROM INFORMATION_SCHEMA.COLUMNS WHERE TABLE_SCHEMA = '");
    query.push_str(database);
    query.push_str("' AND TABLE_NAME = '");
    query.push_str(table);
    query.push_str("'");

    Ok(query)
}
pub fn grab_columnnames(
    table: &str,
    database: &str,
    select: Vec<&str>,
) -> std::result::Result<String, Box<dyn std::error::Error>> {
    let mut query =
        String::from("SELECT COLUMN_NAME FROM INFORMATION_SCHEMA.COLUMNS WHERE TABLE_SCHEMA = '");
    query.push_str(database);
    query.push_str("' AND TABLE_NAME = '");
    query.push_str(table);
    query.push_str("'");
    query.push_str("And COLUMN_NAME != 'INTERNAL_PRIMARY_KEY'");
    query.push_str("And COLUMN_NAME != 'GPS_ID'");
    query.push_str("And COLUMN_NAME != 'X_COORD'");
    query.push_str("And COLUMN_NAME != 'Y_COORD'");
    query.push_str("And COLUMN_NAME != 'Attachment'");
    if select[0]!="*"{
        query.push_str("And COLUMN_NAME in ( ");
        for i in 0..select.len() {
            query.push_str("'");
            query.push_str(select[i]);
            query.push_str("'");
            if i != select.len() - 1 {
                query.push_str(", ");
            }
        }
        query.push_str(")");
    }
    //let stmt: Vec<String> = conn.query_map(query, |datatype|datatype)?; //??
    Ok(query)
}
pub fn grab_all_columnames(
    table: &str,
    database: &str,
    select: Vec<&str>,
) -> std::result::Result<String, Box<dyn std::error::Error>> {
    let mut query =
        String::from("SELECT COLUMN_NAME FROM INFORMATION_SCHEMA.COLUMNS WHERE TABLE_SCHEMA = '");
    query.push_str(database);
    query.push_str("' AND TABLE_NAME = '");
    query.push_str(table);
    query.push_str("'");
    if select[0]!="*"{
        query.push_str("And COLUMN_NAME in ( ");
        for i in 0..select.len() {
            query.push_str("'");
            query.push_str(select[i]);
            query.push_str("'");
            if i != select.len() - 1 {
                query.push_str(", ");
            }
        }
        query.push_str(")");
    }
    //let stmt: Vec<String> = conn.query_map(query, |datatype|datatype)?; //??
    Ok(query)
}
pub fn grab_columnnames_schema(
    table: &str,
    database: &str,
) -> std::result::Result<String, Box<dyn std::error::Error>> {
    let mut query =
        String::from("SELECT COLUMN_NAME FROM INFORMATION_SCHEMA.COLUMNS WHERE TABLE_SCHEMA = '");
    query.push_str(database);
    query.push_str("' AND TABLE_NAME = '");
    query.push_str(table);
    query.push_str("'");
    //let stmt: Vec<String> = conn.query_map(query, |datatype|datatype)?; //??
    Ok(query)
}

fn query_table(
    conn: &mut PooledConn,
    table: &str,
    whereclause: &str,
    database: &str,
    columntypes: Vec<String>,
) -> std::result::Result<Vec<Vec<String>>, Box<dyn std::error::Error>> {
    //    let stmt: Vec<Vec<String>> = conn.query_map(query, |(col1)|{
    //        let mut row: Vec<String> = Vec::new();
    //        row.push(col1);
    //    })?; //??

    let mut stmt = Vec::new();
    for i in 0..columntypes.len() {
        let mut query = String::from("SELECT ");
        query.push_str(&columntypes[i]);
        query.push_str(" FROM ");
        query.push_str(database);
        query.push_str(".");
        query.push_str(table);
        if whereclause != "" {
            query.push_str(" WHERE ");
            query.push_str(whereclause);
        }
        let row = conn
            .query_map(query.clone(), |columntypes: String| columntypes)
            .unwrap();

        stmt.push(row);
        query.clear();
    }
    //let mut fixedstmt:Vec<Vec<String>>=vec![&columntypes.len(), &stmt.len()];
    Ok(stmt)
}
fn deconstruct_where(whereclause: &str)-> (String,String){
    let wherestring= whereclause.to_string();
    println!("where string before{:?}",wherestring);
    let wheresplit  = wherestring.split(".");
    let wherevec= wheresplit.collect::<Vec<&str>>();
    let wheresplitequal= wherevec[0].split("=");
    let wherevec2= wheresplitequal.collect::<Vec<&str>>();
    //split by . and =
    //get values to the right of the .
    //grab valies from left and right of equal sign
    
    
    let parent= wherevec2[0].to_string();
    let child= wherevec2[1].to_string();

    (parent,child)
    
    

}

pub fn build_json_recursive(
    conn: &mut PooledConn,
    database: &str,
    table: &str,
    depth: i32,
    current_depth: i32,
    whereclause: &str,
) -> serde_json::Value{
    let relationships=query_unique_relationships(table).unwrap();
    //println!("relationships: {:?}", relationships);
    //get both parts of tuple
    let stmt= relationships.0;
    let parent= relationships.1;
    let result=exec_query_unique_relationships(conn, &stmt, parent, database).unwrap();

    let relationdrilldown=initialize_db_table(database, table, result).unwrap();
    println!("relationdrilldown: {:?}", relationdrilldown);
    //println!("result: {:?}", result);

    let select=vec!["*"];
    let columns_stmt = grab_columnnames(table, database, select).unwrap();
    let columns = exec_map(conn, &columns_stmt).unwrap();
    let select=vec!["*"];
    let query= query_tables(
            table,
            conn,
            &whereclause,
            &database,
            select,
        );
    let mut recordcount=0;
    if let Some(row) = query.get(1) {
        recordcount = row.len();
        //println!("recordcount: {}", recordcount);
    }
    let select=vec!["*"];
    let json=build_json(query.clone(), database, table, conn, select);
    

    let search=&relationdrilldown.unique_relation;
    let mut childvec=Vec::new();
    let mut parchild:Vec<(String,String)>=Vec::new();
    println!("search: {:?}", search);
    for i in 0..search.len(){
        let child_table=&search[i].child_table;
        childvec.push(child_table);

        let child_colum= &search[i].child_column;
        let parent_column= &search[i].parent_column;
        

        parchild.push((child_colum.to_string(),parent_column.to_string()));
    }
    println!("parchild: {:?}", parchild);
    let mut jsondata:serde_json::Value=json!({});

    for x in 0..recordcount {
        let mut jsonarray:serde_json::Value = json!({});
        for i in 0..query.len() {
            jsonarray[&columns[i]] = query[i][x].clone().into();

            for u in 0..parchild.len(){
                if current_depth < depth && parchild.len()>0 &&columns[i] == parchild[u].1 {
                    let where_child= format!("{}='{}'", parchild[u].0, query[i][x]);
                    let json=build_json_recursive(conn, database, childvec[u], depth, current_depth+1, where_child.as_str());
                    println!("json: {:?}", json);
                    if json==json!({}){
                        jsonarray[&columns[i]] = query[i][x].clone().into();
                    }
                    else{
                        jsonarray[&parchild[u].0] = json;
                    }

                }
            }

        }
        jsondata[&x.to_string()] = jsonarray;
    }
    jsondata











    //let where_deconstructed = deconstruct_where(whereclause);
    //let parentcolumn = where_deconstructed.0;
    //let childcolumn = where_deconstructed.1;
    //let columns_stmt = grab_columnnames(table, database, select).unwrap();
    //let columns = exec_map(conn, &columns_stmt).unwrap();
    //let mut recordcount = 0;
    //if let Some(row) = queryresult.get(1) {
    //    recordcount = row.len();
    //    //println!("recordcount: {}", recordcount);
    //}

}

pub fn build_json_withchild(
    queryresult: Vec<Vec<String>>,
    //childresult: Vec<Vec<String>>,
    child_table:&str,
    whereclause: &str,
    database: &str,
    table: &str,
    conn: &mut PooledConn,
    select: Vec<&str>,
) -> serde_json::Value{
    let where_deconstructed = deconstruct_where(whereclause);
    let parentcolumn = where_deconstructed.0;
    let childcolumn = where_deconstructed.1;
    let columns_stmt = grab_columnnames(table, database, select).unwrap();
    let columns = exec_map(conn, &columns_stmt).unwrap();
    let mut recordcount = 0;
    if let Some(row) = queryresult.get(1) {
        recordcount = row.len();
        //println!("recordcount: {}", recordcount);
    }

    let mut jsondata:serde_json::Value = json!({});
    for x in 0..recordcount {
        let mut jsonarray:serde_json::Value = json!({});
        for i in 0..queryresult.len() {
            jsonarray[&columns[i]] = queryresult[i][x].clone().into();
            //println!("columns[i]: {}", columns[i]);
            //println!("parentcolumn: {}", parentcolumn);
            if columns[i] == parentcolumn{
                //write value of child column above into json
                jsonarray[&childcolumn]=  queryresult[i][x].clone().into();

                

                let where_child= format!("{}='{}'", childcolumn, queryresult[i][x]);

                let childtablequery=query_tables(child_table, conn, &where_child, database, vec!["*"]);
                //check if childtablequery is empty
                //if empty, write empty json into json
                //
                
                if childtablequery[0].is_empty(){
                    jsonarray[&childcolumn] = queryresult[i][x].clone().into();
                }
                else{
                    let jsonchild=build_json(childtablequery,&database, &child_table, conn, vec!["*"]);
                    //write both childvalue and jsonchild into json
                    jsonarray[&childcolumn] = jsonchild;
                }

            }

        }
        jsondata[&x.to_string()] = jsonarray;
    }
    jsondata
}
pub fn build_json(
    queryresult: Vec<Vec<String>>,
    database: &str,
    table: &str,
    conn: &mut PooledConn,
    select: Vec<&str>,
) -> serde_json::Value{
    //let columns = gettablecol::get_table_col(conn,table, database).unwrap();
    let columns_stmt = grab_columnnames(table, database, select).unwrap();
    let columns = exec_map(conn, &columns_stmt).unwrap();
    let mut recordcount = 0;
    if let Some(row) = queryresult.get(1) {
        recordcount = row.len();
        println!("recordcount: {}", recordcount);
    }

    let mut jsondata = json!({});
    for x in 0..recordcount {
        let mut jsonarray = json!({});
        for i in 0..queryresult.len() {
            jsonarray[&columns[i]] = queryresult[i][x].clone().into();
        }
        jsondata[&x.to_string()] = jsonarray;
    }
    jsondata
}
pub fn query_table_schema(
    columns: Vec<String>,
    types: Vec<String>,
    constraints: Vec<(String,String)>,

)-> serde_json::Value{
    let mut jsondata = json!({});
    println!("Constraints: {:?}", constraints);
    
    for x in 0..columns.len() {
        let mut jsonarray = json!({});
        jsonarray["column_name"] = columns[x].clone().into();
        jsonarray["column_type"] = types[x].clone().into();
        //loop over constraint name to see if it matches column name
        //if constraints[x] != "" {
        //match column name to constraint column if column name is in constraint name
        //
        for y in constraints.iter(){
            if y.0 == columns[x]{

                //    jsonarray["constraint_name"] = y.1.clone().into();
                //}
                jsonarray["constraint_name"] = y.1.clone().into();
            }
        }

        //}
        jsondata[&x.to_string()] = jsonarray;
    }
    jsondata

}
pub fn grab_tablenames(
    database: &str,
) -> std::result::Result<String, Box<dyn std::error::Error>> {
    let mut query = String::from("SELECT TABLE_NAME FROM INFORMATION_SCHEMA.TABLES WHERE TABLE_SCHEMA = '");
    query.push_str(database);
    query.push_str("'");
    Ok(query)
}

pub fn exec_grab_tablenames(
    conn: &mut PooledConn,
    query: &str,
) -> std::result::Result<Vec<String>, Box<dyn std::error::Error>> {
    let stmt: Vec<String> = conn.query_map(query, |data| data)?;
    Ok(stmt)
}

pub fn json_table_names(
    queryresult: Vec<String>,
    database: &str,
) -> serde_json::Value{
    let mut jsondata = json!({});
    for x in 0..queryresult.len() {
        let mut jsonarray = json!({});
        jsonarray["table_name"] = queryresult[x].clone().into();
        jsonarray["table_schema"] = database.clone().into();
        jsondata[&x.to_string()] = jsonarray;
    }
    jsondata
}
pub fn query_database_schema(
    tablecoltypestorage: Vec<(&str,Vec<String>, Vec<String>, Vec<(String,String)>)>,
    database: &str,
    ) -> serde_json::Value{
    let mut jsondata = json!({});
    for x in 0..tablecoltypestorage.len() {
        let mut jsonarray = json!({});
        jsonarray["table_name"] = tablecoltypestorage[x].0.clone().into();
        jsonarray["table_schema"] = database.clone().into();
        jsonarray["columns"] = tablecoltypestorage[x].1.clone().into();
        jsonarray["types"] = tablecoltypestorage[x].2.clone().into();
        //grab first string in tuple
        let (constraitcol, constraintname) = tablecoltypestorage[x].3[0].clone();
        //put both into constraint column
        jsonarray["constraints"] = constraintname.clone().into();
        jsonarray["constraint_column"] = constraitcol.clone().into();
        

        jsondata[&x.to_string()] = jsonarray;
    }
    jsondata
}

pub fn query_relationship(database: &str,parent_table:&str, child_table:&str, where_clause:&str) -> std::result::Result<String, Box<dyn std::error::Error>> {
    let mut query = String::from("SELECT * FROM ");
    query.push_str(database);
    query.push_str(".");
    query.push_str(parent_table);
    query.push_str(" LEFT JOIN ");
    query.push_str(database);
    query.push_str(".");
    query.push_str(child_table);
    query.push_str(" ON ");
    query.push_str(where_clause);
    Ok(query)
}

pub fn query_constraints(
    table: &str,
    database: &str,
) -> std::result::Result<String, Box<dyn std::error::Error>> {
    let mut query = String::from("SELECT COLUMN_NAME, CONSTRAINT_NAME FROM INFORMATION_SCHEMA.KEY_COLUMN_USAGE WHERE TABLE_SCHEMA = '");
    query.push_str(database);
    query.push_str("' AND TABLE_NAME = '");
    query.push_str(table);
    query.push_str("'");
    Ok(query)
}
fn query_unique_relationships(parent_table:&str) -> std::result::Result<(String, &str), Box<dyn std::error::Error>> {
    let mut query = String::from("SELECT TARGETED_DATABASE, parent_table, child_table, where_clause FROM Relationships.relationships WHERE parent_table = '");
    query.push_str(parent_table);
    query.push_str("'");
    Ok((query, parent_table))
}

fn count_unique_relationships(parent_table:&str) -> std::result::Result<String, Box<dyn std::error::Error>> {
    let mut query = String::from("SELECT COUNT(1) FROM Relationships.relationships WHERE parent_table = '");
    query.push_str(parent_table);
    query.push_str("'");
    Ok(query)
}
fn grab_child_unique_relationships(parent_table:&str) -> std::result::Result<String, Box<dyn std::error::Error>> {
    let mut query = String::from("SELECT unique child_table FROM Relationships.relationships WHERE parent_table = '");
    query.push_str(parent_table);
    query.push_str("'");
    Ok(query)
}

fn exec_count_unique_relationships(
    conn: &mut PooledConn,
    query: &str,
) -> std::result::Result<Vec<String>, Box<dyn std::error::Error>> {
    let stmt:Vec<String> = conn.query_map(query, |data| data)?;
    Ok(stmt)
}
fn exec_query_unique_relationships(
    conn: &mut PooledConn,
    query: &str,
    parent_table: &str,
    database: &str,
) -> std::result::Result<Vec<UniqueRelation>, Box<dyn std::error::Error>> {
    //create stmt that is a Vec<Vec<String>>
    //let count_stmt: Vec<u32> = conn.query_map(query, |data| data)?;
    let countstmt = count_unique_relationships(parent_table).unwrap();
    let count = exec_count_unique_relationships(conn, &countstmt).unwrap()[0].parse::<u32>().unwrap();

    //let unique_child_tables = grab_child_unique_relationships(parent_table).unwrap();
    //let unique_child_tables = exec_count_unique_relationships(conn, &unique_child_tables).unwrap();
    let relationshiptablecolumns=vec!["TARGETED_DATABASE".to_string(), "parent_table".to_string(), "child_table".to_string(), "where_clause".to_string()];
    let mut vectors: Vec<Vec<String>> = Vec::new();
    for column in relationshiptablecolumns.iter(){
        let mut query = String::from("SELECT ");
        query.push_str(column);
        query.push_str(" FROM Relationships.relationships WHERE parent_table = '");
        query.push_str(parent_table);
        query.push_str("'");
        let stmt: Vec<String> = conn.query_map(&query, |data| data)?;
        vectors.push(stmt);
    }
    //put 

    //let mut unique_relation: Vec<UniqueRelation> = Vec::new();
        let relation= create_uniques(vectors).unwrap();

    


    Ok(relation)
}


pub fn initialize_db_table(
    database: &str,
    table: &str,
    relationships: Vec<UniqueRelation>,
) -> std::result::Result<DatabaseTableDrilldown, Box<dyn std::error::Error>> {

    let count= relationships.len();
    let DatabaseTableDrilldown = DatabaseTableDrilldown::new(database, table, relationships, count.try_into().unwrap());

    //search uniquw relatoinships via child table
    //


    Ok(DatabaseTableDrilldown)


}

pub fn create_uniques(relationship_vec:Vec<Vec<String>>) -> std::result::Result<Vec<UniqueRelation>, Box<dyn std::error::Error>> {
    let mut unique_relation: Vec<UniqueRelation> = Vec::new();
        for j in 0..relationship_vec[0].len() {
            //let relationship = relationship_vec[i][j].clone();

            let database= relationship_vec[0][j].clone();
            let parent_table= relationship_vec[1][j].clone();
            let child_table= relationship_vec[2][j].clone();
            let where_clause= relationship_vec[3][j].clone();
            let split: Vec<&str> = where_clause.split("=").collect();
            let parent_column= split[0].to_string();
            let child_column= split[1].to_string();

            let relation= UniqueRelation::new(&database, &parent_table, &parent_column,&child_table,  &child_column);
            unique_relation.push(relation);
        }
    
    println!("{:?}", unique_relation);
    Ok(unique_relation)
}



pub struct TableColTypeStorage {
    pub table_name: String,
    pub columns: Vec<String>,
    pub types: Vec<String>,
    pub constraints: Vec<(String,String)>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct QueryResults {
    pub records: Vec<Vec<String>>,
}
impl QueryResults {
    pub fn new() -> Self {
        QueryResults {
            records: Vec::new(),
        }
    }
    pub fn add_record(&mut self, record: Vec<String>) {
        self.records.push(record);
    }
}


#[derive(Debug)]
pub struct DatabaseTableDrilldown{
    pub database: String,
    pub table_name: String,
    pub unique_relation: Vec<UniqueRelation>,
    pub relation_count: i32,
    pub unique_relation_count: i32,

}
impl DatabaseTableDrilldown{
    pub fn new(database:&str, table_name: &str,unique_relation:Vec<UniqueRelation>, relation_count: i32) -> Self{
        DatabaseTableDrilldown{
            database: database.to_string(),
            table_name: table_name.to_string(),
            unique_relation,
            relation_count,
            unique_relation_count: relation_count,
        }
    }
    pub fn add_relationship(&mut self, unique_relation: UniqueRelation){
        self.unique_relation.push(unique_relation);
        self.unique_relation_count;
    }
    pub fn add_count(&mut self){
        self.relation_count=self.unique_relation.len().try_into().unwrap();
        self.unique_relation_count=self.unique_relation.len().try_into().unwrap();
    }
    
}

#[derive(Debug)]
pub struct UniqueRelation{
    pub parent_table: String,
    pub parent_column: String,
    pub child_table: String,
    pub child_column: String,
    pub database: String,
}
impl UniqueRelation{
    pub fn new(database: &str, parent_table: &str, parent_column: &str, child_table: &str, child_column: &str) -> Self{

        UniqueRelation{
            database: database.to_string(),
            parent_table: parent_table.to_string(),
            parent_column: parent_column.to_string(),
            child_table: child_table.to_string(),
            child_column: child_column.to_string(),
        }



    }

}



