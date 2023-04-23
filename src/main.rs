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


#[actix_web::main]
async fn main() {
    let server = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/method", web::post().to(method))
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
    else{
        println!("No method selected");
    }

    println!("{}",result);
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("pages/methodsuccess.html"))

}
#[derive(Serialize, Deserialize)]
pub struct FormData {
    method: String,
    database: String,
    table: String,
    csvpath: std::path::PathBuf,
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
type Column=Vec<String>;

