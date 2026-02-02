use crate::PooledConn;
use diesel::prelude::*;
use diesel::sql_query;
use diesel::sql_types::Text;
use diesel::QueryableByName;
use serde_json::json;

//use crate::{connkey, dbconnect};
pub mod displayquery;
pub fn query_tables(
    table: &str,
    conn: &mut PooledConn,
    whereclause: &str,
    database: &str,
    select: Vec<&str>,
    expanded: bool,
) -> Vec<Vec<String>> {
    if !expanded {
        let columns_stmt = grab_columnnames(table, database, select).unwrap();
        let columns = exec_map(conn, &columns_stmt).unwrap();

        let querydata = query_table(conn, table, whereclause, database, columns).unwrap();

        querydata
    } else {
        let columns_stmt = grab_all_columnames(table, database, select).unwrap();
        let columns = exec_map(conn, &columns_stmt).unwrap();

        let querydata = query_table(conn, table, whereclause, database, columns).unwrap();
        querydata
    }
}

pub fn exec_map(
    conn: &mut PooledConn,
    query: &str,
) -> std::result::Result<Vec<String>, Box<dyn std::error::Error>> {
    #[derive(QueryableByName)]
    struct SingleString {
        #[diesel(sql_type = Text)]
        column_name: String,
    }

    let rows: Vec<SingleString> = sql_query(query).load(conn)?;
    Ok(rows.into_iter().map(|r| r.column_name).collect())
}

pub fn exec_map_tuple(
    conn: &mut PooledConn,
    query: &str,
) -> std::result::Result<Vec<(String, String)>, Box<dyn std::error::Error>> {
    #[derive(QueryableByName)]
    struct TwoStrings {
        #[diesel(sql_type = Text)]
        column_name: String,
        #[diesel(sql_type = Text)]
        constraint_name: String,
    }

    let rows: Vec<TwoStrings> = sql_query(query).load(conn)?;
    Ok(rows
        .into_iter()
        .map(|r| (r.column_name, r.constraint_name))
        .collect())
}
pub fn grab_columntypes(
    table: &str,
    database: &str,
) -> std::result::Result<String, Box<dyn std::error::Error>> {
    // Postgres: use data_type and alias it to column_name so exec_map can
    // deserialize it with the SingleString { column_name } struct.
    let mut query = String::from(
        "SELECT data_type AS column_name FROM information_schema.columns WHERE table_schema = '",
    );
    query.push_str(database);
    query.push_str("' AND TABLE_NAME = '");
    query.push_str(table);
    query.push_str("'");
    query.push_str(" AND column_name != 'internal_primary_key'");
    query.push_str(" AND column_name != 'gps_id'");
    query.push_str(" AND column_name != 'x_coord'");
    query.push_str(" AND column_name != 'y_coord'");
    query.push_str(" AND column_name != 'attachment'");

    Ok(query)
}

pub fn grab_all_columntypes(
    table: &str,
    database: &str,
) -> std::result::Result<String, Box<dyn std::error::Error>> {
    let mut query = String::from(
        "SELECT data_type AS column_name FROM information_schema.columns WHERE table_schema = '",
    );
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
    let mut query = String::from(
        "SELECT data_type AS column_name FROM information_schema.columns WHERE table_schema = '",
    );
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
    if select[0] != "*" {
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
    if select[0] != "*" {
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
pub fn retrieveattachmentstmt(
    table: &str,
    database: &str,
    id: &str,
) -> std::result::Result<String, Box<dyn std::error::Error>> {
    let mut query = String::from("SELECT Attachment FROM ");
    query.push_str(database);
    query.push_str(".");
    query.push_str(table);
    query.push_str("_GPS");
    query.push_str(" WHERE INTERNAL_PRIMARY_KEY= ");
    query.push_str(id);
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
    for col in columntypes.iter() {
        let mut query = String::from("SELECT ");
        // Cast to text so Diesel can load everything as String
        query.push_str(col);
        query.push_str("::text AS value FROM ");
        query.push_str(database);
        query.push_str(".");
        query.push_str(table);
        if !whereclause.is_empty() {
            query.push_str(" WHERE ");
            query.push_str(whereclause);
        }

        #[derive(QueryableByName)]
        struct ValueRow {
            #[diesel(sql_type = Text)]
            value: String,
        }

        let rows: Vec<ValueRow> = sql_query(query.clone()).load(conn)?;
        let row_vals: Vec<String> = rows.into_iter().map(|r| r.value).collect();
        stmt.push(row_vals);
    }

    Ok(stmt)
}
fn deconstruct_where(whereclause: &str) -> (String, String) {
    let wherestring = whereclause.to_string();
    println!("where string before{:?}", wherestring);
    let wheresplit = wherestring.split(".");
    let wherevec = wheresplit.collect::<Vec<&str>>();
    let wheresplitequal = wherevec[0].split("=");
    let wherevec2 = wheresplitequal.collect::<Vec<&str>>();
    //split by . and =
    //get values to the right of the .
    //grab valies from left and right of equal sign

    let parent = wherevec2[0].to_string();
    let child = wherevec2[1].to_string();

    (parent, child)
}

pub fn build_json_withchild(
    queryresult: Vec<Vec<String>>,
    //childresult: Vec<Vec<String>>,
    child_table: &str,
    whereclause: &str,
    database: &str,
    table: &str,
    conn: &mut PooledConn,
    select: Vec<&str>,
) -> serde_json::Value {
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

    let mut jsondata: serde_json::Value = json!({});
    for x in 0..recordcount {
        let mut jsonarray: serde_json::Value = json!({});
        for i in 0..queryresult.len() {
            jsonarray[&columns[i]] = queryresult[i][x].clone().into();
            //println!("columns[i]: {}", columns[i]);
            //println!("parentcolumn: {}", parentcolumn);
            if columns[i] == parentcolumn {
                //write value of child column above into json
                jsonarray[&childcolumn] = queryresult[i][x].clone().into();

                let where_child = format!("{}='{}'", childcolumn, queryresult[i][x]);

                let childtablequery =
                    query_tables(child_table, conn, &where_child, database, vec!["*"], false);
                //check if childtablequery is empty
                //if empty, write empty json into json
                //

                if childtablequery[0].is_empty() {
                    jsonarray[&childcolumn] = queryresult[i][x].clone().into();
                } else {
                    let jsonchild = build_json(
                        childtablequery,
                        &database,
                        &child_table,
                        conn,
                        vec!["*"],
                        false,
                    );
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
    expand: bool,
) -> serde_json::Value {
    //let columns = gettablecol::get_table_col(conn,table, database).unwrap();

    let mut columns_stmt = String::new();
    let mut columns: Vec<String> = Vec::new();
    if !expand {
        columns_stmt = grab_columnnames(table, database, select).unwrap();
        columns = exec_map(conn, &columns_stmt).unwrap();
    } else if expand {
        columns_stmt = grab_all_columnames(table, database, vec!["*"]).unwrap();
        columns = exec_map(conn, &columns_stmt).unwrap();
    }

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
    constraints: Vec<(String, String)>,
) -> serde_json::Value {
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
        for y in constraints.iter() {
            if y.0 == columns[x] {
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
pub fn grab_tablenames(database: &str) -> std::result::Result<String, Box<dyn std::error::Error>> {
    let mut query =
        String::from("SELECT TABLE_NAME FROM INFORMATION_SCHEMA.TABLES WHERE TABLE_SCHEMA = '");
    query.push_str(database);
    query.push_str("'");
    Ok(query)
}

pub fn exec_grab_tablenames(
    conn: &mut PooledConn,
    query: &str,
) -> std::result::Result<Vec<String>, Box<dyn std::error::Error>> {
    #[derive(QueryableByName)]
    struct SingleStringRow {
        #[diesel(sql_type = Text)]
        table_name: String,
    }

    let rows: Vec<SingleStringRow> = sql_query(query).load(conn)?;
    Ok(rows.into_iter().map(|r| r.table_name).collect())
}

pub fn json_table_names(queryresult: Vec<String>, database: &str) -> serde_json::Value {
    let mut jsondata = json!({});
    for x in 0..queryresult.len() {
        let mut jsonarray = json!({});

        jsonarray["table_name"] = queryresult[x].clone().into();
        let table = queryresult[x].clone();
        jsonarray["table_schema"] = database.into();
        jsondata[&table.to_string()] = jsonarray;
    }
    jsondata
}
pub fn query_database_schema(
    tablecoltypestorage: Vec<(&str, Vec<String>, Vec<String>, Vec<(String, String)>)>,
    database: &str,
) -> serde_json::Value {
    let mut jsondata = json!({});
    for x in 0..tablecoltypestorage.len() {
        let mut jsonarray = json!({});
        jsonarray["table_name"] = tablecoltypestorage[x].0.into();
        let table = tablecoltypestorage[x].0;
        jsonarray["table_schema"] = database.into();
        jsonarray["columns"] = tablecoltypestorage[x].1.clone().into();
        jsonarray["types"] = tablecoltypestorage[x].2.clone().into();
        //grab first string in tuple
        let (constraitcol, constraintname) = tablecoltypestorage[x].3[0].clone();
        //put both into constraint column
        jsonarray["constraints"] = constraintname.clone().into();
        jsonarray["constraint_column"] = constraitcol.clone().into();

        jsondata[table.to_string()] = jsonarray;
    }
    jsondata
}

pub fn query_relationship(
    database: &str,
    parent_table: &str,
    child_table: &str,
    where_clause: &str,
) -> std::result::Result<String, Box<dyn std::error::Error>> {
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
fn query_unique_relationships(
    parent_table: &str,
) -> std::result::Result<(String, &str), Box<dyn std::error::Error>> {
    let mut query = String::from("SELECT TARGETED_DATABASE, parent_table, child_table, where_clause FROM Relationships.relationships WHERE parent_table = '");
    query.push_str(parent_table);
    query.push_str("'");
    Ok((query, parent_table))
}

//fn count_unique_relationships(parent_table:&str) -> std::result::Result<String, Box<dyn std::error::Error>> {
//    let mut query = String::from("SELECT COUNT(1) FROM Relationships.relationships WHERE parent_table = '");
//    query.push_str(parent_table);
//    query.push_str("'");
//    Ok(query)
//}
//
//
//fn exec_count_unique_relationships(
//    conn: &mut PooledConn,
//    query: &str,
//) -> std::result::Result<Vec<String>, Box<dyn std::error::Error>> {
//    let stmt:Vec<String> = conn.query_map(query, |data| data)?;
//    Ok(stmt)
//}
fn exec_query_unique_relationships(
    conn: &mut PooledConn,
    _query: &str,
    parent_table: &str,
    _database: &str,
) -> std::result::Result<Vec<UniqueRelation>, Box<dyn std::error::Error>> {
    #[derive(QueryableByName)]
    struct RelRow {
        #[diesel(sql_type = Text)]
        #[allow(non_snake_case)]
        TARGETED_DATABASE: String,
        #[diesel(sql_type = Text)]
        parent_table: String,
        #[diesel(sql_type = Text)]
        child_table: String,
        #[diesel(sql_type = Text)]
        where_clause: String,
    }

    let mut query = String::from(
        "SELECT TARGETED_DATABASE, parent_table, child_table, where_clause FROM Relationships.relationships WHERE parent_table = '",
    );
    query.push_str(parent_table);
    query.push_str("'");

    let rows: Vec<RelRow> = sql_query(&query).load(conn)?;

    let uniques = rows
        .into_iter()
        .map(|r| {
            let split: Vec<&str> = r.where_clause.split('=').collect();
            let parent_column = split.get(0).unwrap_or(&"").to_string();
            let child_column = split.get(1).unwrap_or(&"").to_string();
            UniqueRelation::new(
                &r.TARGETED_DATABASE,
                &r.parent_table,
                &parent_column,
                &r.child_table,
                &child_column,
            )
        })
        .collect();

    Ok(uniques)
}

pub fn initialize_db_table(
    database: &str,
    table: &str,
    relationships: Vec<UniqueRelation>,
) -> std::result::Result<DatabaseTableDrilldown, Box<dyn std::error::Error>> {
    let count = relationships.len();
    let database_table_drilldown =
        DatabaseTableDrilldown::new(database, table, relationships, count.try_into().unwrap());

    Ok(database_table_drilldown)
}

pub fn create_uniques(
    relationship_vec: Vec<Vec<String>>,
) -> std::result::Result<Vec<UniqueRelation>, Box<dyn std::error::Error>> {
    let mut unique_relation: Vec<UniqueRelation> = Vec::new();
    for j in 0..relationship_vec[0].len() {
        //let relationship = relationship_vec[i][j].clone();

        let database = relationship_vec[0][j].clone();
        let parent_table = relationship_vec[1][j].clone();
        let child_table = relationship_vec[2][j].clone();
        let where_clause = relationship_vec[3][j].clone();
        let split: Vec<&str> = where_clause.split("=").collect();
        let parent_column = split[0].to_string();
        let child_column = split[1].to_string();

        let relation = UniqueRelation::new(
            &database,
            &parent_table,
            &parent_column,
            &child_table,
            &child_column,
        );
        unique_relation.push(relation);
    }

    println!("{:?}", unique_relation);
    Ok(unique_relation)
}
pub fn build_json_recursive(
    conn: &mut PooledConn,
    database: &str,
    table: &str,
    depth: i32,
    current_depth: i32,
    whereclause: &str,
) -> serde_json::Value {
    let relationships = query_unique_relationships(table).unwrap();

    let stmt = relationships.0;
    let parent = relationships.1;
    let result = exec_query_unique_relationships(conn, &stmt, parent, database).unwrap();

    let relationdrilldown = initialize_db_table(database, table, result).unwrap();

    let select = vec!["*"];
    let columns_stmt = grab_columnnames(table, database, select).unwrap();
    let columns = exec_map(conn, &columns_stmt).unwrap();
    let select = vec!["*"];
    let query = query_tables(table, conn, &whereclause, &database, select, false);
    let mut recordcount = 0;
    if let Some(row) = query.get(1) {
        recordcount = row.len();
        //println!("recordcount: {}", recordcount);
    }

    let search = &relationdrilldown.unique_relation;
    let mut childvec = Vec::new();
    let mut parchild: Vec<(String, String)> = Vec::new();
    for i in 0..search.len() {
        let child_table = &search[i].child_table;
        childvec.push(child_table);

        let child_colum = &search[i].child_column;
        let parent_column = &search[i].parent_column;

        parchild.push((child_colum.to_string(), parent_column.to_string()));
    }
    let mut jsondata: serde_json::Value = json!({});

    for x in 0..recordcount {
        let mut jsonarray: serde_json::Value = json!({});
        for i in 0..query.len() {
            jsonarray[&columns[i]] = query[i][x].clone().into();

            for u in 0..parchild.len() {
                if current_depth < depth && parchild.len() > 0 && columns[i] == parchild[u].1 {
                    let where_child = format!("{}='{}'", parchild[u].0, query[i][x]);
                    let json = build_json_recursive(
                        conn,
                        database,
                        childvec[u],
                        depth,
                        current_depth + 1,
                        where_child.as_str(),
                    );
                    if json == json!({}) {
                        jsonarray[&columns[i]] = query[i][x].clone().into();
                    } else {
                        jsonarray[&parchild[u].0] = json;
                    }
                }
            }
        }
        jsondata[&x.to_string()] = jsonarray;
    }
    jsondata
}

#[derive(Debug)]
pub struct DatabaseTableDrilldown {
    pub database: String,
    pub table_name: String,
    pub unique_relation: Vec<UniqueRelation>,
    pub relation_count: i32,
    pub unique_relation_count: i32,
}
impl DatabaseTableDrilldown {
    pub fn new(
        database: &str,
        table_name: &str,
        unique_relation: Vec<UniqueRelation>,
        relation_count: i32,
    ) -> Self {
        DatabaseTableDrilldown {
            database: database.to_string(),
            table_name: table_name.to_string(),
            unique_relation,
            relation_count,
            unique_relation_count: relation_count,
        }
    }
    pub fn add_relationship(&mut self, unique_relation: UniqueRelation) {
        self.unique_relation.push(unique_relation);
        self.unique_relation_count;
    }
    pub fn add_count(&mut self) {
        self.relation_count = self.unique_relation.len().try_into().unwrap();
        self.unique_relation_count = self.unique_relation.len().try_into().unwrap();
    }
}

#[derive(Debug)]
pub struct UniqueRelation {
    pub parent_table: String,
    pub parent_column: String,
    pub child_table: String,
    pub child_column: String,
    pub database: String,
}
impl UniqueRelation {
    pub fn new(
        database: &str,
        parent_table: &str,
        parent_column: &str,
        child_table: &str,
        child_column: &str,
    ) -> Self {
        UniqueRelation {
            database: database.to_string(),
            parent_table: parent_table.to_string(),
            parent_column: parent_column.to_string(),
            child_table: child_table.to_string(),
            child_column: child_column.to_string(),
        }
    }
}
