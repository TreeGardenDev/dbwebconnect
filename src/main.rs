use clap::Parser;
use csv::Reader;
use mysql::prelude::*;
use mysql::*;
use serde::{Deserialize, Serialize};
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
pub mod pushdata;
pub mod getfields;
pub mod tablecreate;
pub mod dbconnect;

#[actix_web::main]
async fn main() {
    let server = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/method", web::post().to(method))
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
        let columns=getfields::read_fields(&form.csvpath.display().to_string());
        pushdata::createtablestruct::read_csv2(&form.csvpath.display().to_string(), form.table.to_string());
    }
    else if form.method=="create"{
        let mut connection=dbconnect::database_connection();
        let tablename=&form.table.to_string();
        let columns=getfields::read_fields(&form.csvpath.display().to_string());
        getfields::read_types(&form.csvpath.display().to_string());
        tablecreate::create_table(&mut connection,&tablename,&columns);
    }

    println!("{}",result);
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("pages/methodsuccess.html"))

}
#[derive(Serialize, Deserialize)]
pub struct FormData {
    method: String,
    table: String,
    csvpath: std::path::PathBuf,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Data2{
    columns: Vec<column>
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
type column=Vec<String>;

struct Table{
    tablename: String,
    columnname:Vec<column>,
}
