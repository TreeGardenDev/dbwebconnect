use actix_web::{web, App,  HttpResponse, HttpServer, Responder, cookie};
use actix_session::{Session,SessionMiddleware, storage::RedisActorSessionStore};
use actix_identity::{CookieIdentityPolicy, IdentityService};
use serde_json::Value;
use std::sync::Mutex;
use crate::createrecord::generateform::UploadForm;
use crate::createrecord::generateform::CreateTable;
//comment
//use crate::createrecord::generateform::CreateRelation;
//use futures_util::TryStreamExt as _;
//use uuid::Uuid;
use actix_multipart::form::{tempfile::TempFileConfig, MultipartForm};
//use actix_multipart::Multipart;
use clap::Parser;
use csv::Reader;
use mysql::*;
use serde::{Deserialize, Serialize};
pub mod insertrecords;
pub mod pushdata;
pub mod getfields;
pub mod tablecreate;
pub mod dbconnect;
pub mod createdatabase;
pub mod querytable;
pub mod createrecord;
pub mod initconnect;
pub mod createrelationship;
pub mod connkey;

#[actix_web::main]
async fn main() {
    //grab the first argument
    let mut args = std::env::args().nth(1).unwrap();
    args.push_str(":8080"); 
    
    let secretkey=cookie::Key::generate();
    let redisconnection=String::from("127.0.0.1:6379");
    let appsess=AppState::new();
    
    let server = HttpServer::new(move|| {
        App::new()
            .wrap(
                SessionMiddleware::new(
                    RedisActorSessionStore::new(&redisconnection),
                    
                    secretkey.clone(),
                )
            )
            
            
            //session cookie
            .app_data(TempFileConfig::default().directory("./tmp"))
            //force users to start at / before going to /main

            .service(
                web::resource("/")
                    .route(web::get().to(getinitializeconnect))
                    .route(web::post().to(postinitializeconnect))
            )
            .route("/main", web::get().to(index))
            .route("/auth", web::post().to(auth))
            .route("/method", web::post().to(method))
            .route("/createtable", web::post().to(createtable))
            .route("/createtable/{database}&table={table}&apikey={apikey}", web::post().to(createtableweb))
            .route("/createdatabase", web::post().to(createnewdb))
            .route("/createdatabase/{database}&apikey={apikey}&{port}&{url}", web::post().to(createnewdbweb))
            .route("/query", web::post().to(query))
            .route("/query/{database}&table={table}&select={select}&where={where}&apikey={api}", web::get().to(querytojson))
            .service(
                web::resource("/create")
                    .route(web::get().to(getcreate))
                    .route(web::post().to(postcreate)),
            
            )
            .route("/insert/{database}&table={table}&apikey={api}", web::post().to(dbinsert))
                
            .route("/create/saveform", web::post().to(saveform))
            .service(
                web::resource("/upload")
                    .route(web::get().to(getupload))
                    .route(web::post().to(postupload)),
            )
            .service(
                web::resource("/createrelation")
                    .route(web::get().to(getcreaterelation))
                    .route(web::post().to(postcreaterelationdefined)),
            )
        

            
//            .route("/insert", web::post().to(method))
 //           .route("/create", web::post().to(method))
    });
    println!("Starting server at {}", args);
    server.bind(args).expect("Can not bind to port 8080").run().await.unwrap();
}
async fn postinitializeconnect(form:web::Form<LinkDataBase> )->impl Responder{
    let _=initconnect::postdatabaseconnection(form.into_inner());
    //post to appdata from here
    
    
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("page.html"))
}
async fn getinitializeconnect()->impl Responder{
    let html=initconnect::getpagehtml();
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}
async fn getcreaterelation()->impl Responder{
    let html=createrecord::generateform::getcreaterelationshipdefined();
    HttpResponse::Ok()
        .body(html)
}
async fn postcreaterelationdefined(form:web::Form<NewRelationShip> )->impl Responder{
    let database =form.database.clone();

    let _ =createrelationship::commitrelationshipdefined(&database,&form.table1, &form.column1, &form.table2, &form.column2, &form.ondelete, &form.onupdate);
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("pages/methodsuccess.html"))
}
async fn index()->impl Responder{
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("page.html"))
 }
async fn getupload()->impl Responder{
    let html=createrecord::generateform::fileinsert();
    HttpResponse::Ok()
        .body(html)
 }
async fn postupload(
    MultipartForm(form):MultipartForm<UploadForm>,
) -> impl Responder {
    let table=&form.table.clone();
    let database=&form.database.clone();


    let file=createrecord::generateform::file_upload(form);

    let _ =pushdata::createtablestruct::read_csv2(&file, table, database);

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("pages/methodsuccess.html"))
}
async fn createtableweb(info:web::Path<(String, String, String)>, body:web::Json<Value>)->impl Responder{

        let valid=connkey::search_apikey(&info.0,&info.2);
        if valid.unwrap()==true{
        let mut conn=dbconnect::internalqueryconnapikey();
        let body=body.into_inner();
        let mut data=Vec::new();
        for (key, value) in body.as_object().unwrap().iter() {
            data.push((key.to_string(),value.to_string()));
        }
        println!("{:?}",data);
        let database=&info.0;
        let table=&info.1;
        let parsed_json=tablecreate::parse_json(data);
        let _=tablecreate::create_table_web(&mut conn, &database,&table,&parsed_json.0, &parsed_json.1);
        

        HttpResponse::Ok()
            .content_type("text/json; charset=utf-8")
            .body("Table Created")
        }
        else{
            
            HttpResponse::Ok()
                .content_type("text/json; charset=utf-8")
                .body("Invalid API Key")
        }

        
        
    
}
async fn dbinsert(info: web::Path<(String,String,String)>, body:web::Json<Value>)->impl Responder{
    let valid=connkey::search_apikey(&info.0,&info.2);
    if valid.unwrap()==true{
    let body=body.into_inner();
    //decode json
    let mut data=Vec::new();
    for (key, value) in body.as_object().unwrap().iter() {
        data.push((key.to_string(),value.to_string()));
    }
    println!("DATA BELOW");
    println!("{:?}",data);

    let database=&info.0;
    let table=&info.1;
    //let apikey=&info.2;

    let mut newtable=insertrecords::TableDef::new();
    newtable.populate(&table, &database);
    println!("{:?}",newtable);

    let valid= newtable.compare_fields(&data);
    if valid {
        let _ =newtable.insert(&data, &table, &database);

    }
    else{
        return HttpResponse::Ok()
        .content_type("text/json; charset=utf-8")
        .body("Invalid Data");
    }



    HttpResponse::Ok()
        .content_type("text/json; charset=utf-8")
        .body("Insert Successful")
    }
    else{
       HttpResponse::Ok()
        .content_type("text/json; charset=utf-8")
        .body("Invalid API Key")
    }
}

async fn createnewdb(form: web::Form<NewDataBase>)->impl Responder{
    let _ =createdatabase::create_database(&form.database.to_string());
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("pages/methodsuccess.html"))
}
async fn createnewdbweb(info: web::Path<(String,String,String,String)>)->impl Responder{
    let valid=connkey::search_apikey_admin(&info.1);
    if valid.unwrap()==true{
    
    let database_name=&info.0;
    let apikey=&info.1;
    let port=&info.2;
    let url=&info.3;
    let _ =createdatabase::create_databaseweb( database_name, apikey);
    HttpResponse::Ok()
        .content_type("text/json; charset=utf-8")
        .body("{'status':'success'}")
    }
    else{
        HttpResponse::Ok()
        .content_type("text/json; charset=utf-8")
        .body("{'status':'failed'}")
    }
}
async fn createtable(MultipartForm(form):MultipartForm<CreateTable>) -> impl Responder{
    let mut connection=dbconnect::database_connection(&form.database.clone().to_string());
    let tablename=&form.table.clone().to_string();
    let file=createrecord::generateform::uploadnewcols(form);
    println!("file here debug: {}",file);
    let columns=getfields::read_fields(&file);
    let types=getfields::read_types(&file);

    let _ =tablecreate::create_table(&mut connection,&tablename,&columns,&types);
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("pages/methodsuccess.html"))
}
async fn method(form: web::Form<FormData>)->impl Responder{
    let result = format!("Method: {} Table: {} CSV: {}", form.method, form.table, form.csvpath.display());
    if form.method=="insert"{
        let _ = pushdata::createtablestruct::read_csv2(&form.csvpath.display().to_string(), &form.table.to_string(), &form.database.to_string());
    }
    if form.method=="create"{
        let mut connection=dbconnect::database_connection(&form.database.to_string());
        let tablename=&form.table.to_string();
        let columns=getfields::read_fields(&form.csvpath.display().to_string());
        let types=getfields::read_types(&form.csvpath.display().to_string());
        let _ =tablecreate::create_table(&mut connection,&tablename,&columns,&types);
    }
    else if form.method=="newdb"{
        createdatabase::create_database(&form.database.to_string());

    }
    else if form.method=="query"{
        let mut connection=dbconnect::database_connection(&form.database.to_string());
        let tablename=&form.table.to_string();
        //let columns=getfields::read_fields(&form.csvpath.display().to_string());
        //let types=getfields::read_types(&form.csvpath.display().to_string());
        let queryresult= querytable::query_tables(&tablename, &mut connection,&form.csvpath.display().to_string(), &form.database.to_string());
        println!("{:?}",queryresult);

    }
    else if form.method=="csv"{
       let mut connection=dbconnect::database_connection(&form.database.to_string()); 
        let _=createrecord::create_session_csv(&mut connection, &form.table.to_string(), &form.database.to_string());
    }
    else{
        println!("No method selected");
    }

    println!("{}",result);
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("pages/methodsuccess.html"))

}
async fn auth(form: web::Form<Auth>)->impl Responder{
    let result = format!("Username: {} Password: {}", form.username, form.password);

    println!("{}",result);
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("page.html"))
}
async fn query(form: web::Form<QueryData>)->impl Responder{
        let mut connection=dbconnect::database_connection(&form.database.to_string());
        let tablename=&form.table.to_string();
        let columns=pushdata::gettablecol::get_table_col(&mut connection, &tablename, &form.database.to_string()).unwrap();
        //let types=getfields::read_types(&form.csvpath.display().to_string());
        let queryresult= querytable::query_tables(&tablename, &mut connection,&form.whereclause.to_string(), &form.database.to_string());
        println!("{:?}",queryresult);
        let html=querytable::displayquery::buildhtml(queryresult, &form.database.to_string(), &form.table.to_string(), columns);
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}
async fn querytojson(info: web::Path<(String,String,String,String,String)>)->impl Responder{
    let mut connection=dbconnect::database_connection(&info.0);

    let valid=connkey::search_apikey(&info.0, &info.4);
    if valid.unwrap()==true{

    let database=&info.0;
    let tablename=&info.1;
    let select=&info.2;
    let whereclause=&info.3;
    let apikey=&info.4;

    let queryresult= querytable::query_tables(&tablename, &mut connection,&whereclause, &database);
    let json= querytable::build_json(queryresult, &database, &tablename, &mut connection);
    println!("JSON BELOW");
    println!("{:?}",json);



    HttpResponse::Ok()
        .content_type("text/json; charset=utf-8")
        .body(json)
    }
    else{
        HttpResponse::Ok()
        .content_type("text/json; charset=utf-8")
        .body("Invalid API Key")
    }
}
async fn getcreate(form: web::Form<NewCsv>)-> impl Responder{
    let mut connection=dbconnect::database_connection(&form.database.to_string());
    let tablename=&form.table.to_string();
    let database=&form.database.to_string();
    let columns=pushdata::gettablecol::get_table_col(&mut connection, &tablename, &form.database.to_string()).unwrap();
    //let _=createrecord::create_record(&mut connection, &form.table.to_string(), &form.database.to_string(), &form.records);
    let html =createrecord::generateform::buildform(database, tablename, columns);
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}
async fn postcreate(form: web::Form<SaveNewCsv>)-> impl Responder{
    let connection=dbconnect::database_connection(&form.database.to_string());
    //let tablename=&form.table.to_string();
    //let database=&form.database.to_string();
   // let columns=pushdata::gettablecol::get_table_col(&mut connection, &tablename, &form.database.to_string()).unwrap();
   // 
    println!("{}, {}, {:?}", form.database, form.table, form.data);
    //println!("{:?}", form.data);
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("pages/methodsuccess.html"))
}
async fn saveform(web::Form(form): web::Form<NewRecord>)-> impl Responder{
    //take form data and print it
    println!("{:?}", form);
    //let mut connection=dbconnect::database_connection(&form.database.to_string());
    //get user input from form data from create function
  //  let newrecord=NewRecord{
  //      records: form.records
  //  };
    //println!("{:?}", newrecord);
//    newrecord.records
    let html=createrecord::generateform::formresponse(form);
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}
#[derive(Serialize, Deserialize)]
pub struct FormData {
    method: String,
    database: String,
    table: String,
    csvpath: std::path::PathBuf,
}
#[derive(Serialize, Deserialize)]
pub struct QueryData {
    database: String,
    table: String,
    whereclause: String,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Data2{
    columns: Vec<Column>
}
#[derive(Debug, PartialEq, Eq)]
struct ColData{
    fields: Vec<String>
}

#[derive(Parser)]
struct CLI{
    pattern: String,
    table: String,
    path:std::path::PathBuf,
}
#[derive(Parser, Serialize, Deserialize)]
pub struct SaveNewCsv{
    database: String,
    table: String,
    data: Vec<String>,
}
#[derive(Parser, Serialize, Deserialize)]
pub struct NewCsv{
    database: String,
    table: String,
}
#[derive(Parser, Serialize,Debug, Deserialize)]
pub struct NewRecord{
    records: Vec<String>,
}
type Column=Vec<String>;

#[derive(Parser, Serialize,Debug, Deserialize)]
pub struct Auth{
    username: String,
    password: String,
}


#[derive(Parser, Serialize,Debug, Deserialize)]
pub struct LinkDataBase{
    dbuser: String,
    dbpass: String,
    dbhost: String,
    dbport: String,

}
#[derive(Parser, Serialize,Debug, Deserialize)]
pub struct NewDataBase{
    database: String,
}

#[derive(Parser, Serialize,Debug, Deserialize)]
pub struct NewRelationShip{
    database: String,
    table1: String,
    column1: String,
    table2: String,
    column2: String,
    onupdate: String,
    ondelete: String,
}
#[derive(Parser, Serialize,Debug, Deserialize)]
pub struct CsvRequestBody{
    data:String,
}
#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct UserData {
    logged_in: bool,
}

//create a struct whose lifetime is the same as the application
struct AppState {
    authenticated:Mutex<bool>,
    apikey: Mutex<String>,
    counter: Mutex<i32>, // <- Mutex is necessary to mutate safely across threads

}
//make struct AppState available to the application
impl AppState {
    fn new() -> Self {
        Self {
            //make counter a time that continuously counts up
            counter: Mutex::new(0), // <- initialize counter
            authenticated: Mutex::new(false),
            apikey: Mutex::new("".to_string()),
        }
    }
    fn disable(&self){
        let mut authenticated=self.authenticated.lock().unwrap();
        *authenticated=false;
    }
    fn enable(&self){
        let mut authenticated=self.authenticated.lock().unwrap();
        *authenticated=true;
    }
    fn authenticate(&self, apikey: String){
        let mut key=self.apikey.lock().unwrap();
        *key=apikey;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_record() {
        let new_record = NewRecord { records: vec!["record1".to_string(), "record2".to_string()] };
        assert_eq!(new_record.records.len(), 2);
    }

    #[test]
    fn test_new_database() {
        let new_database = NewDataBase { database: "my_database".to_string() };
        assert_eq!(new_database.database, "my_database");
    }

    #[test]
    fn test_new_relationship() {
        let new_relationship = NewRelationShip {
            database: "my_database".to_string(),
            table1: "table1".to_string(),
            column1: "column1".to_string(),
            table2: "table2".to_string(),
            column2: "column2".to_string(),
            onupdate: "CASCADE".to_string(),
            ondelete: "CASCADE".to_string(),
        };
        assert_eq!(new_relationship.database, "my_database");
        assert_eq!(new_relationship.table1, "table1");
        assert_eq!(new_relationship.column1, "column1");
        assert_eq!(new_relationship.table2, "table2");
        assert_eq!(new_relationship.column2, "column2");
        assert_eq!(new_relationship.onupdate, "CASCADE");
        assert_eq!(new_relationship.ondelete, "CASCADE");
    }
}
