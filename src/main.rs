use clap::Parser;
use csv::Reader;
use mysql::*;
use serde::{Deserialize, Serialize};
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
pub mod pushdata;
pub mod getfields;
pub mod tablecreate;
pub mod dbconnect;
pub mod createdatabase;
pub mod querytable;
pub mod createrecord;

#[actix_web::main]
async fn main() {
    let server = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/method", web::post().to(method))
            .route("/query", web::post().to(query))
            .route("/create", web::post().to(create))
//            .route("/insert", web::post().to(method))
 //           .route("/create", web::post().to(method))
    });
    println!("Starting server at localhost:8080");
    server.bind("192.168.0.230:8080").expect("Can not bind to port 8080").run().await.unwrap();
}
async fn index()->impl Responder{
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("page.html"))
        //.content_type("text/css")
        //.body(include_str!("pages/mystyle.css"))
 }
async fn method(form: web::Form<FormData>)->impl Responder{
    let result = format!("Method: {} Table: {} CSV: {}", form.method, form.table, form.csvpath.display());
    if form.method=="insert"{
        //let columns=getfields::read_fields(&form.csvpath.display().to_string());
        let _ = pushdata::createtablestruct::read_csv2(&form.csvpath.display().to_string(), form.table.to_string(), &form.database.to_string());
    }
    else if form.method=="create"{
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
async fn create(form: web::Form<NewCsv>)->impl Responder{
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
async fn saveform(web::Form(form): web::Form<NewRecord>)->Vec<String>{
    let mut connection=dbconnect::database_connection(&form.database.to_string());
    //get user input from form data from create function
    let newrecord=NewRecord{
        database: form.database,
        table: form.table,
        records: form.records,
    };
    println!("{:?}", newrecord);
    newrecord.records
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
pub struct NewCsv{
    database: String,
    table: String,
}
#[derive(Parser, Serialize,Debug, Deserialize)]
pub struct NewRecord{
    database: String,
    table: String,
    records: Vec<String>,
}
type Column=Vec<String>;

