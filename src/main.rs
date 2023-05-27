use actix_web::{web, App,  HttpResponse, HttpServer, Responder};
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
pub mod pushdata;
pub mod getfields;
pub mod tablecreate;
pub mod dbconnect;
pub mod createdatabase;
pub mod querytable;
pub mod createrecord;
pub mod initconnect;
pub mod createrelationship;

#[actix_web::main]
async fn main() {
    //grab the first argument
    let mut args = std::env::args().nth(1).unwrap();
    args.push_str(":8080"); 
    
    let server = HttpServer::new(|| {
        App::new()
            .app_data(TempFileConfig::default().directory("./tmp"))
            .service(
                web::resource("/")
                    .route(web::get().to(getinitializeconnect))
                    .route(web::post().to(postinitializeconnect))
            )
            .route("/main", web::get().to(index))
            .route("/auth", web::post().to(auth))
            .route("/method", web::post().to(method))
            .route("/createtable", web::post().to(createtable))
            .route("/createdatabase", web::post().to(createnewdb))
            .route("/createdatabase/{database}/{databaseuser}/{databasepword}/{port}/{url}", web::post().to(createnewdbweb))
            .route("/query", web::post().to(query))
            .service(
                web::resource("/create")
                    .route(web::get().to(getcreate))
                    .route(web::post().to(postcreate)),
            
            )
                
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
//    let creds=initconnect::postdatabaseconnection(form.into_inner());
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
async fn createnewdb(form: web::Form<NewDataBase>)->impl Responder{
    let _ =createdatabase::create_database(&form.database.to_string());
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("pages/methodsuccess.html"))
}
async fn createnewdbweb(info: web::Path<(String,String,String,String,String)>)->impl Responder{
    let database_name=&info.0;
    let database_user=&info.1;
    let database_pword=&info.2;
    let port=&info.3;
    let url=&info.4;
    let _ =createdatabase::create_databaseweb( database_name, database_user, database_pword, port, url);
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("pages/methodsuccess.html"))
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