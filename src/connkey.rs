//Purpose of this file:
//1. create an apikey that is usable by user to connect to the server
//2. search for the apikey in the database
//3. if apikey found run request
//4. if apikey not found return error

use data_encoding::HEXUPPER;
use mysql::prelude::Queryable;
use ring::digest::{Context, SHA256};

use crate::dbconnect;
pub struct ApiKey {
    pub apikey: String,
    database: String,
}

impl ApiKey {
    pub fn new() -> ApiKey {

        ApiKey { apikey:String::new(),
            database:String::new(),}

    }
    pub fn populatekey(&mut self, database: String) {
        let salt=std::env::args().nth(3).expect("no salt provided");
        let mut apikey=String::new();
        let mut ctx = Context::new(&SHA256);
        ctx.update(database.as_bytes());
        ctx.update(salt.as_bytes());
        let digest = ctx.finish();
        apikey.push_str(&HEXUPPER.encode(digest.as_ref()));

        self.apikey = apikey;
        self.database = database;
    }
}

pub fn search_apikey(database: &str, apikey: &str) -> Result<bool, Box<dyn std::error::Error>>{
    println!("database: {}", database);
    println!("apikey: {}", apikey);
    let mut conn=dbconnect::internalqueryconnapikey();
    let mut stmt=String::from("SELECT apikey FROM ApiKey.apikeys WHERE databaseuser= '");
    stmt.push_str(&database);
    stmt.push_str("'");


    let mut keyvec:Vec<String> =Vec::new();

    let _ = conn.query_map(stmt, |apikey| {
        keyvec.push(apikey);
    })?;
    println!("{:?}", keyvec);

    

    if apikey == keyvec[0] {
        return Ok(true);
    }
    else {
       return Ok(false)
    }
}
pub fn search_apikey_admin(apikey:&str)-> Result<bool, Box<dyn std::error::Error>>{
    
    let mut conn=dbconnect::internalqueryconnapikey();
    let stmt=String::from("SELECT apikey FROM apikeys WHERE databaseuser= 'root'");
    let mut keyvec:Vec<String> =Vec::new();

    let _ = conn.query_map(stmt, |apikey| {
        
        keyvec.push(apikey);
    })?;

    

    if apikey == keyvec[0] {
        return Ok(true);
    }
    else {
       return Ok(false)
    }
}

pub fn insert_apikey(database: String, hash:String) -> Result<(), Box<dyn std::error::Error>> {
    let mut conn=dbconnect::internalqueryconnapikey();
    let mut apikey=ApiKey::new();
    apikey.populatekey(database);
    let mut stmt=String::from("INSERT INTO apikeys (apikey,databaseuser, databasepasshash) VALUES (");
    stmt.push_str("'");
    stmt.push_str(&apikey.apikey);
    stmt.push_str("', '");
    stmt.push_str(&apikey.database);
    stmt.push_str("', '");
    stmt.push_str(&hash);

    stmt.push_str("')");

    conn.query_drop(stmt)?;

    Ok(())
}
//generate a random password for the database
//
pub fn random_password() -> String {
    let mut password=String::new();
    //want to generate a random number with 19 digits
    for _ in 0.. 19{
        //set rng to random number between 0 and 9
        let rng:u8 = rand::random::<u8>() % 10;

        password.push_str(&rng.to_string());
    }
    password
}
//read the hash from the database
pub fn read_hash(apikey: String) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut conn=dbconnect::internalqueryconnapikey();
    let mut stmt=String::from("SELECT databaseuser, databasepasshash FROM apikeys WHERE apikey= ");
    stmt.push_str(&apikey);
    let mut keyvec:Vec<String> =Vec::new();

    let query = conn.query_map(stmt, |user| {
        
        keyvec.push(user);


    })?;
    

    if query.len() > 0 {
        //return Ok(keyvec[0].clone());
        return Ok(keyvec);
    }
    else {
        return Err("No hash found".into());
    }
}

