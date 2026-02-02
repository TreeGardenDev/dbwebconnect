//Purpose of this file:
//1. create an apikey that is usable by user to connect to the server
//2. search for the apikey in the database
//3. if apikey found run request
//4. if apikey not found return error

use data_encoding::HEXUPPER;
use diesel::prelude::*;
use diesel::sql_query;
use ring::digest::{Context, SHA256};

use crate::dbconnect;
pub struct ApiKey {
    pub apikey: String,
    database: String,
}

impl ApiKey {
    pub fn new() -> ApiKey {
        ApiKey {
            apikey: String::new(),
            database: String::new(),
        }
    }
    pub fn populatekey(&mut self, database: String) {
        // Legacy API-key generation no longer relies on a CLI-provided salt.
        // Use a fixed salt to avoid panics when no CLI args are present.
        let salt = std::env::var("APIKEY_SALT").unwrap_or_else(|_| "legacy-salt".to_string());
        let mut apikey = String::new();
        let mut ctx = Context::new(&SHA256);
        ctx.update(database.as_bytes());
        ctx.update(salt.as_bytes());
        let digest = ctx.finish();
        apikey.push_str(&HEXUPPER.encode(digest.as_ref()));

        self.apikey = apikey;
        self.database = database;
    }
}

pub fn execute_apikey(stmt: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut conn = dbconnect::internalqueryconnapikey();
    #[derive(QueryableByName)]
    struct ApiKeyRow {
        #[diesel(sql_type = diesel::sql_types::Text)]
        apikey: String,
    }

    let rows: Vec<ApiKeyRow> = sql_query(stmt).load(&mut conn)?;
    if rows.is_empty() {
        Err("No API key found".into())
    } else {
        Ok(rows[0].apikey.clone())
    }
}

pub fn generate_apikey(database: &String) -> Result<String, Box<dyn std::error::Error>> {
    let mut stmt = String::from("SELECT apikey FROM apikeys WHERE databaseuser= '");
    stmt.push_str(&database);
    stmt.push_str("'");
    Ok(stmt)
}
pub fn search_apikey(database: &str, apikey: &str) -> Result<bool, Box<dyn std::error::Error>> {
    let mut conn = dbconnect::internalqueryconnapikey();
    let mut stmt = String::from("SELECT apikey FROM apikeys WHERE databaseuser= '");
    stmt.push_str(&database);
    stmt.push_str("'");

    #[derive(QueryableByName)]
    struct ApiKeyRow {
        #[diesel(sql_type = diesel::sql_types::Text)]
        apikey: String,
    }

    let rows: Vec<ApiKeyRow> = sql_query(stmt).load(&mut conn)?;
    if rows.is_empty() {
        Ok(false)
    } else {
        Ok(apikey == rows[0].apikey)
    }
}
pub fn search_apikey_admin(apikey: &str) -> Result<bool, Box<dyn std::error::Error>> {
    let mut conn = dbconnect::internalqueryconnapikey();
    let stmt = String::from("SELECT apikey FROM apikeys WHERE databaseuser= 'root'");
    #[derive(QueryableByName)]
    struct ApiKeyRow {
        #[diesel(sql_type = diesel::sql_types::Text)]
        apikey: String,
    }

    let rows: Vec<ApiKeyRow> = sql_query(stmt).load(&mut conn)?;
    if rows.is_empty() {
        Ok(false)
    } else {
        Ok(apikey == rows[0].apikey)
    }
}

pub fn insert_apikey(database: String, hash: String) -> Result<String, Box<dyn std::error::Error>> {
    let mut conn = dbconnect::internalqueryconnapikey();
    let mut apikey = ApiKey::new();
    apikey.populatekey(database);
    let mut stmt =
        String::from("INSERT INTO apikeys (apikey,databaseuser, databasepasshash) VALUES (");
    stmt.push_str("'");
    stmt.push_str(&apikey.apikey);
    stmt.push_str("', '");
    stmt.push_str(&apikey.database);
    stmt.push_str("', '");
    stmt.push_str(&hash);

    stmt.push_str("')");

    sql_query(stmt).execute(&mut conn)?;

    Ok(apikey.apikey)
}
//generate a random password for the database
//
pub fn random_password() -> String {
    let mut password = String::new();
    //want to generate a random number with 19 digits
    for _ in 0..19 {
        //set rng to random number between 0 and 9
        let rng: u8 = rand::random::<u8>() % 10;

        password.push_str(&rng.to_string());
    }
    password
}
//read the hash from the database
pub fn read_hash(apikey: String) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut conn = dbconnect::internalqueryconnapikey();
    let mut stmt =
        String::from("SELECT databaseuser, databasepasshash FROM apikeys WHERE apikey= ");
    stmt.push_str(&apikey);
    #[derive(QueryableByName)]
    struct HashRow {
        #[diesel(sql_type = diesel::sql_types::Text)]
        databaseuser: String,
        #[diesel(sql_type = diesel::sql_types::Text)]
        databasepasshash: String,
    }

    let rows: Vec<HashRow> = sql_query(stmt).load(&mut conn)?;

    if let Some(row) = rows.get(0) {
        Ok(vec![row.databaseuser.clone(), row.databasepasshash.clone()])
    } else {
        Err("No hash found".into())
    }
}
