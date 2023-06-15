use actix_web::{web, App,  HttpResponse, HttpServer, Responder, cookie};
use actix_session::{SessionMiddleware, storage::RedisActorSessionStore};
//use actix_identity::{CookieIdentityPolicy, IdentityService};
use serde_json::Value;
use crate::createrecord::generateform::UploadForm;
use crate::createrecord::generateform::CreateTable;
//use crate::createrecord::generateform::CreateRelation;
//use futures_util::TryStreamExt as _;
//use uuid::Uuid;
use actix_multipart::form::{tempfile::TempFileConfig, MultipartForm};
//use actix_multipart::Multipart;
use clap::Parser;
use csv::Reader;
use mysql::*;
use serde::{Deserialize, Serialize};
pub mod update;
pub mod delete;
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
    let mut args = std::env::args().nth(1).unwrap();
    args.push_str(":8080"); 
    
    let secretkey=cookie::Key::generate();
    let redisconnection=String::from("127.0.0.1:6379");
    
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
            .route("/createdatabase/{database}&apikey={apikey}", web::post().to(createnewdbweb))
            .route("/query", web::post().to(query))
            .route("/query/{database}&table={table}&select={select}&where={where}&apikey={api}", web::get().to(querytojson))
            .service(
                web::resource("/create")
                    .route(web::get().to(getcreate))
                    .route(web::post().to(postcreate)),
            
            )
            .route("/insert/{database}&table={table}&apikey={api}", web::post().to(dbinsert))
            .route("/updaterecord/{database}&table={table}&apikey={api}", web::post().to(dbupdaterecord))
                
            .route("/create/saveform", web::post().to(saveform))
            .service(
                web::resource("/upload")
                    .route(web::get().to(getupload))
                    .route(web::post().to(postupload)),
            )
            .route("/relationship/{database}&apikey={api}", web::post().to(createrelationshipweb))
            .route("/deleterecord/{database}&table={table}&apikey={api}", web::post().to(deleterecord))
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
async fn postinitializeconnect(form:web::Form<ApiKey> )->impl Responder{
    let valid=connkey::search_apikey_admin(&form.apikey).unwrap();
    if valid==true{
        //let _=initconnect::postdatabaseconnection(form.into_inner());
        HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(include_str!("page.html"))
    }
    else{
        HttpResponse::Ok()
            .content_type("text/json; charset=utf-8")
            .body("{\"error\":\"Invalid API Key\"}")
    }
    //let _=initconnect::postdatabaseconnection(form.into_inner());
    //post to appdata from here
    
    
   // HttpResponse::Ok()
   //     .content_type("text/html; charset=utf-8")
   //     .body(include_str!("page.html"))
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
async fn createrelationshipweb(info:web::Path<(String, String)>, body:web::Json<Value>)->impl Responder{
    let valid=connkey::search_apikey(&info.0,&info.1);
    if valid.unwrap()==true{
        
        let body=body.into_inner();
        let mut data=Vec::new();
        for (key, value) in body.as_object().unwrap().iter() {
            let parsed=createrelationship::parse_json(value.to_string());
            println!("{:?}",parsed);
            data.push((key.to_string(),parsed));
        }
        let relationship=createrelationship::createrelationshipfromweb(&info.0, data);
        let _=createrelationship::commitrelationshipfromweb(relationship);
        

        HttpResponse::Ok()
            .content_type("text/json; charset=utf-8")
            .body("Status: 200 Relationship Created")
    }
    else{

        HttpResponse::Ok()
            .content_type("text/json; charset=utf-8")
            .body("Status: 400 Invalid API Key")
    }
}
async fn deleterecord(info:web::Path<(String, String, String)>, body:web::Json<Value>)->impl Responder{
    let valid=connkey::search_apikey(&info.0,&info.2);
    if valid.unwrap()==true{
        let mut conn=dbconnect::internalqueryconnapikey();
        let body=body.into_inner();
        let mut data=Vec::new();
        for (key, value) in body.as_object().unwrap().iter() {
            data.push((key.to_string(),value.to_string()));
        }
        let database=&info.0;
        let table=&info.1;
        //let parsed_json=tablecreate::parse_json(data);
        let _=delete::deleterecord(&mut conn, &database,&table, data);
        

        HttpResponse::Ok()
            .content_type("text/json; charset=utf-8")
            .body("Status: 200 Record Deleted")
    }
    else{

        HttpResponse::Ok()
            .content_type("text/json; charset=utf-8")
            .body("Status: 400 Invalid API Key")
    }
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
        let stmt =newtable.insert(&data, &table, &database);
        let _=insertrecords::exec_insert(stmt);

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
async fn dbupdaterecord(info: web::Path<(String,String,String)>, body:web::Json<Value>)->impl Responder{
    let valid=connkey::search_apikey(&info.0,&info.2);
    if valid.unwrap()==true{
        let conn=dbconnect::internalqueryconnapikey();
        let body=body.into_inner();
        let mut data=Vec::new();
        for (key, value) in body.as_object().unwrap().iter() {
            //strip out quotes
            let value=value.to_string().replace("\"","");
        data.push((key.to_string(),value.to_string()));
        }
        let database=&info.0;
        let table=&info.1;
        let statement=update::updaterecord(database, table, data);
        let _=update::executeupdaterecord(conn, &statement);


        HttpResponse::Ok()
            .content_type("text/json; charset=utf-8")
            .body("Update Successful")
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
async fn createnewdbweb(info: web::Path<(String,String)>)->impl Responder{
    let valid=connkey::search_apikey_admin(&info.1);
    if valid.unwrap()==true{
    
    let database_name=&info.0;
    //let apikey=&info.1;
    let _ =createdatabase::create_databaseweb(database_name);
    HttpResponse::Ok()
        .content_type("text/json; charset=utf-8")
        .body("Success 200: Database Created")
    }
    else{
        HttpResponse::Ok()
        .content_type("text/json; charset=utf-8")
        .body("Err 500: Not a valid API Key")
    }
}
async fn createtable(MultipartForm(form):MultipartForm<CreateTable>) -> impl Responder{
    let mut connection=dbconnect::internalqueryconn();
    let database=&form.database.clone();
    let tablename=&form.table.clone().to_string();
    let file=createrecord::generateform::uploadnewcols(form);
    println!("file here debug: {}",file);
    let columns=getfields::read_fields(&file);
    let types=getfields::read_types(&file);

    let _ =tablecreate::create_table(&mut connection,&database, &tablename,&columns,&types);
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
        let datbase=&form.database.to_string();
        let tablename=&form.table.to_string();
        let columns=getfields::read_fields(&form.csvpath.display().to_string());
        let types=getfields::read_types(&form.csvpath.display().to_string());
        let _ =tablecreate::create_table(&mut connection,&datbase, &tablename,&columns,&types);
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

    let valid=connkey::search_apikey(&info.0, &info.4);
    if valid.unwrap()==true{
        let mut connection=dbconnect::internalqueryconn();

    let database=&info.0;
    let tablename=&info.1;
    //let select=&info.2;
    let whereclause=&info.3;
    //let apikey=&info.4;

    let queryresult= querytable::query_tables(&tablename, &mut connection,&whereclause, &database);
    println!("Query result below");
    println!("{:?}",queryresult);
    let json= querytable::build_json(queryresult, &database, &tablename, &mut connection);



    HttpResponse::Ok()
        .content_type("text/json; charset=utf-8")
        .body(json.to_string())
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
    //let connection=dbconnect::database_connection(&form.database.to_string());
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

#[derive(Serialize, Deserialize)]
pub struct ApiKey{
    apikey: String,
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



#[cfg(test)]
mod tests {
    use crate::insertrecords::TableDef;

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
    #[test]
    fn test_insert_record(){
        let database="unit_tests";
        let table="testinsertupdatedelete";
        let data=vec![("col1".to_string(), "50".to_string()),("col2".to_string(), "Test Addition".to_string())];
        let mut newrecord=TableDef::new();
        //newrecord.populate(table, database);
        newrecord.table_fields.push("col1".to_string());
        newrecord.table_fields.push("col2".to_string());
        newrecord.table_types.push("int(11)".to_string());
        newrecord.table_types.push("varchar(255)".to_string());

        assert_eq!(newrecord.table_types, vec!["int(11)".to_string(), "varchar(255)".to_string()]);
        assert_eq!(newrecord.table_fields, vec!["col1".to_string(), "col2".to_string()]);
        println!("{:?}", newrecord);
        let mut validvec:Vec<bool>=Vec::new();
        for i in 0..newrecord.table_fields.len(){
            validvec.push(false);
            for j in 0..data.len(){
                if newrecord.table_fields[i]==data[j].0{
                    validvec[i]=true;
                }
            }
        }
        for i in 0..validvec.len(){
            assert_eq!(validvec[i], true);
        }
        let valid= newrecord.compare_fields(&data);
        assert_eq!(valid, true);
        let insert =newrecord.insert(&data, &table, &database);
        assert_eq!(insert, String::from("INSERT INTO unit_tests.testinsertupdatedelete (col1, col2) VALUES (50, 'Test Addition')"));

        

    }
    #[test]
    fn test_update_record(){
        let database="unit_tests";
        let table="testinsertupdatedelete";
        let mut data:Vec<(String,String)>=Vec::new();
        data.push(("INTERNAL_PRIMARY_KEY".to_string(), "1".to_string()));
        data.push(("col1".to_string(), "50".to_string()));
        data.push(("col2".to_string(), "Changed".to_string()));

        let update=update::updaterecord(database, table, data);
        //assert_eq!(update.unwrap(), String::from("Success"));
        assert_eq!(update, String::from("UPDATE unit_tests.testinsertupdatedelete SET col1= \"50\", col2= \"Changed\" WHERE INTERNAL_PRIMARY_KEY=1"));
        
    }
}
