use crate::createrecord::generateform::CreateTable;
use crate::relationships::Relationship_Builder;
use crate::createrecord::generateform::UploadForm;
use actix_multipart::form::{tempfile::TempFileConfig, MultipartForm};
use actix_session::{storage::RedisActorSessionStore, SessionMiddleware};
use actix_web::{cookie, web, App, HttpResponse, HttpServer, Responder};
use clap::Parser;
use csv::Reader;
use mysql::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;
//use crate::createrecord::generateform::CreateRelation;
//use actix_identity::{CookieIdentityPolicy, IdentityService};
//use futures_util::TryStreamExt as _;
//use uuid::Uuid;
//use actix_multipart::Multipart;
pub mod connkey;
pub mod createdatabase;
pub mod createrecord;
pub mod createrelationship;
pub mod dbconnect;
pub mod delete;
pub mod getfields;
pub mod initconnect;
pub mod insertrecords;
pub mod pushdata;
pub mod querytable;
pub mod tablecreate;
pub mod update;
pub mod relationships;

#[actix_web::main]
async fn main() {
    let mut args = std::env::args().nth(1).unwrap();
    args.push_str(":8080");
    //let pword=std::env::args().nth(2).unwrap();

    let secretkey = cookie::Key::generate();
    let redisconnection = String::from("127.0.0.1:6379");

    let server = HttpServer::new(move || {
        App::new()
            .wrap(SessionMiddleware::new(
                RedisActorSessionStore::new(&redisconnection),
                secretkey.clone(),
            ))
            //session cookie
            .app_data(TempFileConfig::default().directory("./tmp"))
            //force users to start at / before going to /main
            .service(
                web::resource("/")
                    .route(web::get().to(getinitializeconnect))
                    .route(web::post().to(postinitializeconnect)),
            )
            .route("/main", web::get().to(index))
            .route("/auth", web::post().to(auth))
            .route("/method", web::post().to(method))
            .route("/createtable", web::post().to(createtable))
            .route(
                "/createtable/{database}&table={table}&gps={gps}&apikey={apikey}",
                web::post().to(createtableweb),
            )
            .route(
                "/droptable/{database}&table={table}&apikey={apikey}",
                web::post().to(droptableweb),
            )
            .route("/createdatabase", web::post().to(createnewdb))
            .route(
                "/createdatabase/{database}&apikey={apikey}",
                web::post().to(createnewdbweb),
            )
            .route("/query", web::post().to(query))
            .route(
                "/query/{database}&table={table}&select={select}&where={where}&apikey={api}",
                web::get().to(querytojson),
            )
            .route(
                "/querytableschema/{database}&table={table}&apikey={api}",
                web::get().to(querytableschema),
            )
            .route(
                "/querydatabase/{database}&expand={expand}&apikey={api}",
                web::get().to(querydatabase),
            )
            .service(
                web::resource("/create")
                    .route(web::get().to(getcreate))
                    .route(web::post().to(postcreate)),
            )
            .route(
                "/insert/{database}&table={table}&apikey={api}",
                web::post().to(dbinsert),
            )
            .route(
                "/updaterecord/{database}&table={table}&apikey={api}",
                web::post().to(dbupdaterecord),
            )
            .route("/create/saveform", web::post().to(saveform))
            .service(
                web::resource("/upload")
                    .route(web::get().to(getupload))
                    .route(web::post().to(postupload)),
            )
            .route(
                "/relationship/{database}&apikey={api}",
                web::post().to(createrelationshipweb),
            )
            .route(
                "relateparent/{database}&parent_table={parent_table}&child_table={child_table}&relationship_name={relationship_name}&apikey={api}",
                web::post().to(createrelationshipparentweb),
            )
            .route(
                "/deleterecord/{database}&table={table}&apikey={api}",
                web::post().to(deleterecord),
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
    server
        .bind(args)
        .expect("Can not bind to port 8080")
        .run()
        .await
        .unwrap();
}
async fn postinitializeconnect(form: web::Form<ApiKey>) -> impl Responder {
    let valid = connkey::search_apikey_admin(&form.apikey).unwrap();
    if valid == true {
        //let _=initconnect::postdatabaseconnection(form.into_inner());
        HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(include_str!("page.html"))
    } else {
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
async fn getinitializeconnect() -> impl Responder {
    let html = initconnect::getpagehtml();
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}
async fn getcreaterelation() -> impl Responder {
    let html = createrecord::generateform::getcreaterelationshipdefined();
    HttpResponse::Ok().body(html)
}
async fn postcreaterelationdefined(form: web::Form<NewRelationShip>) -> impl Responder {
    let database = form.database.clone();

    let _ = createrelationship::commitrelationshipdefined(
        &database,
        &form.table1,
        &form.column1,
        &form.table2,
        &form.column2,
        &form.ondelete,
        &form.onupdate,
    );
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("pages/methodsuccess.html"))
}
async fn index() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("page.html"))
}
async fn getupload() -> impl Responder {
    let html = createrecord::generateform::fileinsert();
    HttpResponse::Ok().body(html)
}
async fn postupload(MultipartForm(form): MultipartForm<UploadForm>) -> impl Responder {
    let table = &form.table.clone();
    let database = &form.database.clone();

    let file = createrecord::generateform::file_upload(form);

    let _ = pushdata::createtablestruct::read_csv2(&file, table, database);

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("pages/methodsuccess.html"))
}
async fn createrelationshipweb(
    info: web::Path<(String, String)>,
    body: web::Json<Value>,
) -> impl Responder {
    let valid = connkey::search_apikey(&info.0, &info.1);
    if valid.unwrap() == true {
        let body = body.into_inner();
        let mut data = Vec::new();
        for (key, value) in body.as_object().unwrap().iter() {
            let parsed = createrelationship::parse_json(value.to_string());
            println!("{:?}", parsed);
            data.push((key.to_string(), parsed));
        }
        let relationship = createrelationship::createrelationshipfromweb(&info.0, data);
        let _ = createrelationship::commitrelationshipfromweb(relationship);

        HttpResponse::Ok()
            .content_type("text/json; charset=utf-8")
            .body("Status: 200 Relationship Created")
    } else {
        HttpResponse::Ok()
            .content_type("text/json; charset=utf-8")
            .body("Status: 400 Invalid API Key")
    }
}
async fn createrelationshipparentweb(
    info: web::Path<(String, String,String,String,String)>,
    body: web::Json<Value>,
) -> impl Responder {

    let valid = connkey::search_apikey(&info.0, &info.4);
    if valid.unwrap() == true {
        let body = body.into_inner();
        let mut conn= dbconnect::internalqueryconn();
        let mut data = Vec::new();
        for (key, value) in body.as_object().unwrap().iter() {
            let parsed = createrelationship::parse_json(value.to_string());
            println!("{:?}", parsed);
            data.push((key.to_string(), parsed));
        }

        let related=relationships::Relationship_Builder::new(&info.0, &info.1, &info.2, &info.3, &data[0].1.clone());


        let stmt=relationships::create_relationship_stmt(&related);
        
        println!("{:?}",stmt);
        let _=relationships::execute_relationship_stmt(&stmt, &mut conn);



        HttpResponse::Ok()
            .content_type("text/json; charset=utf-8")
            .body("Status: 200 Relationship Created")
    } else {
        HttpResponse::Ok()
            .content_type("text/json; charset=utf-8")
            .body("Status: 400 Invalid API Key")
    }
}
async fn deleterecord(
    info: web::Path<(String, String, String)>,
    body: web::Json<Value>,
) -> impl Responder {
    let valid = connkey::search_apikey(&info.0, &info.2);
    if valid.unwrap() == true {
        let mut conn = dbconnect::internalqueryconnapikey();
        let body = body.into_inner();
        let mut data = Vec::new();
        for (key, value) in body.as_object().unwrap().iter() {
            data.push((key.to_string(), value.to_string()));
        }
        let database = &info.0;
        let table = &info.1;
        //let parsed_json=tablecreate::parse_json(data);
        let statement = delete::deleterecord(&database, &table, data);
        let _ = delete::exec_statement(&mut conn, &statement.unwrap());

        HttpResponse::Ok()
            .content_type("text/json; charset=utf-8")
            .body("Status: 200 Record Deleted")
    } else {
        HttpResponse::Ok()
            .content_type("text/json; charset=utf-8")
            .body("Status: 400 Invalid API Key")
    }
}
async fn createtableweb(
    info: web::Path<(String, String,String, String)>,
    body: web::Json<Value>,
) -> impl Responder {
    println!("{:?}", &info.3);
    println!("{:?}", &info.0);
    let valid = connkey::search_apikey(&info.0, &info.3);
    if valid.unwrap() == true {
        let mut conn = dbconnect::internalqueryconnapikey();
        let body = body.into_inner();
        let mut data = Vec::new();
        for (key, value) in body.as_object().unwrap().iter() {
            data.push((key.to_string(), value.to_string()));
        }
        println!("{:?}", data);
        let database = &info.0;
        let table = &info.1;
        let gps=&info.2;
        //convert gps to bool
        let gps=gps.parse::<bool>().unwrap();

        let parsed_json = tablecreate::parse_json(data);
        if gps==true{
            let stmt = tablecreate::create_table_web_gps(
                &database,
                &table,
                &parsed_json.0,
                &parsed_json.1,
            );
            if stmt.starts_with("Invalid"){
                return HttpResponse::Ok()
                    .content_type("text/json; charset=utf-8")
                    .body(stmt);
            }
            let _ = tablecreate::exec_statement(&mut conn, &stmt);
        }else{
            let stmt = tablecreate::create_table_web(
                &database,
                &table,
                &parsed_json.0,
                &parsed_json.1,
            );
            //check if statement says invalid as the first word
            if stmt.starts_with("Invalid"){
                return HttpResponse::Ok()
                    .content_type("text/json; charset=utf-8")
                    .body(stmt);
            }
            let _ = tablecreate::exec_statement(&mut conn, &stmt);
        }

        //let _ = tablecreate::create_table_web(
        //    &mut conn,
        //    &database,
        //    &table,
        //    &parsed_json.0,
        //    &parsed_json.1,
        //);

        HttpResponse::Ok()
            .content_type("text/json; charset=utf-8")
            .body("Table Created")
    } else {
        HttpResponse::Ok()
            .content_type("text/json; charset=utf-8")
            .body("Invalid API Key")
    }
}
async fn droptableweb(
    info: web::Path<(String, String, String)>,
    body: web::Json<Value>,
) -> impl Responder {
    let valid = connkey::search_apikey(&info.0, &info.2);
    if valid.unwrap() == false {
        return HttpResponse::Ok()
            .content_type("text/json; charset=utf-8")
            .body("Invalid API Key");
    }
    let mut conn = dbconnect::internalqueryconn();
    let body = body.into_inner();
    //let mut data=Vec::new();
    let mut backup = false;
    for (_, value) in body.as_object().unwrap().iter() {
        //    data.push((key.to_string(),value.to_string()));
        backup = value.as_bool().unwrap();
        //
    }
    println!("{:?}", backup);
    let database = &info.0;
    let table = &info.1;
    if backup == true {
        let backup = delete::generate_backup(database, table);
        let _ = delete::exec_statement(&mut conn, &backup.unwrap());
    }
    //let _=droptable::droptable(&mut conn, &database,&table, false);
    let statement = delete::droptable(&database, &table);
    let _ = delete::exec_statement(&mut conn, &statement.unwrap());

    HttpResponse::Ok()
        .content_type("text/json; charset=utf-8")
        .body("Table Dropped")
}
async fn dbinsert(
    info: web::Path<(String, String, String)>,
    body: web::Json<Vec<Value>>,
) -> impl Responder {
    let valid = connkey::search_apikey(&info.0, &info.2);
    if valid.unwrap() == true {
        let body = body.into_inner();
        let mut storagevec:Vec<Vec<(String,String)>> = Vec::new();
        for record in body.iter(){
            let mut data = Vec::new();
            for (key, value) in record.as_object().unwrap().iter() {
                data.push((key.to_string(), value.to_string()));
            }
            storagevec.push(data);
        }
        println!("{:?}", storagevec);

        let database = &info.0;
        let table = &info.1;
        //let apikey=&info.2;

        let mut newtable = insertrecords::TableDef::new();
        newtable.populate(&table, &database);
        let valid = newtable.compare_fields(&storagevec);

        if valid {
            let stmt = newtable.insert(&storagevec, &table, &database);
            let _ = insertrecords::exec_insert(stmt);
            //println!("{:?}", stmt);
        } else {
            return HttpResponse::Ok()
                .content_type("text/json; charset=utf-8")
                .body("Invalid Data");
        }
        

        HttpResponse::Ok()
            .content_type("text/json; charset=utf-8")
            .body("Insert Successful")
    } else {
        HttpResponse::Ok()
            .content_type("text/json; charset=utf-8")
            .body("Invalid API Key")
    }
}
//async fn dbinsert_gps(
//    info: web::Path<(String, String, String)>,
//    body: web::Json<Vec<Value>>,
//) -> impl Responder {
//    let valid = connkey::search_apikey(&info.0, &info.2);
//    if valid.unwrap() == true {
//        let body = body.into_inner();
//        let mut storagevec:Vec<Vec<(String,String)>> = Vec::new();
//        for record in body.iter(){
//            let mut data = Vec::new();
//            for (key, value) in record.as_object().unwrap().iter() {
//                data.push((key.to_string(), value.to_string()));
//            }
//            storagevec.push(data);
//        }
//        println!("{:?}", storagevec);
//
//        let database = &info.0;
//        let table = &info.1;
//        //let apikey=&info.2;
//
//        let mut newtable = insertrecords::TableDef::new();
//        newtable.populate(&table, &database);
//        let valid = newtable.compare_fields(&storagevec);
//
//        if valid {
//            let stmt = newtable.insert_gps(&storagevec, &table, &database);
//            let _ = insertrecords::exec_insert(stmt);
//            //println!("{:?}", stmt);
//        } else {
//            return HttpResponse::Ok()
//                .content_type("text/json; charset=utf-8")
//                .body("Invalid Data");
//        }
//        
//
//        HttpResponse::Ok()
//            .content_type("text/json; charset=utf-8")
//            .body("Insert Successful")
//    } else {
//        HttpResponse::Ok()
//            .content_type("text/json; charset=utf-8")
//            .body("Invalid API Key")
//    }
//}
async fn dbupdaterecord(
    info: web::Path<(String, String, String)>,
    body: web::Json<Vec<Value>>,
) -> impl Responder {
    let valid = connkey::search_apikey(&info.0, &info.2);
    if valid.unwrap() == true {
        let mut conn = dbconnect::internalqueryconnapikey();
        let body = body.into_inner();
        let mut storagevec:Vec<Vec<(String,String)>> = Vec::new();
        //let mut data = Vec::new();
        for record in body.iter(){
            let mut data = Vec::new();
            for (key, value) in record.as_object().unwrap().iter() {
                let strkey=key.as_str();
                let strvalue=value.as_str().unwrap();
                data.push((strkey.to_string(), strvalue.to_string()));
            }
            storagevec.push(data);
        }
        let database = &info.0;
        let table = &info.1;
        let statement = update::updaterecord(database, table, storagevec);
        for statement in statement.iter(){
            let _ = update::executeupdaterecord(&mut conn, &statement);
        }

        HttpResponse::Ok()
            .content_type("text/json; charset=utf-8")
            .body("Update Successful")
    } else {
        HttpResponse::Ok()
            .content_type("text/json; charset=utf-8")
            .body("Invalid API Key")
    }
}

async fn createnewdb(form: web::Form<NewDataBase>) -> impl Responder {
    let _ = createdatabase::create_database(&form.database.to_string());
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("pages/methodsuccess.html"))
}
async fn createnewdbweb(info: web::Path<(String, String)>) -> impl Responder {
    let valid = connkey::search_apikey_admin(&info.1);
    if valid.unwrap() == true {
        let database_name = &info.0;
        //let apikey=&info.1;
        let _ = createdatabase::create_databaseweb(database_name);
        HttpResponse::Ok()
            .content_type("text/json; charset=utf-8")
            .body("Success 200: Database Created")
    } else {
        HttpResponse::Ok()
            .content_type("text/json; charset=utf-8")
            .body("Err 500: Not a valid API Key")
    }
}
async fn createtable(MultipartForm(form): MultipartForm<CreateTable>) -> impl Responder {
    let mut connection = dbconnect::internalqueryconn();
    let database = &form.database.clone();
    let tablename = &form.table.clone().to_string();
    let file = createrecord::generateform::uploadnewcols(form);
    println!("file here debug: {}", file);
    let columns = getfields::read_fields(&file);
    let types = getfields::read_types(&file);

    let _ = tablecreate::create_table(&mut connection, &database, &tablename, &columns, &types);
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("pages/methodsuccess.html"))
}
async fn method(form: web::Form<FormData>) -> impl Responder {
    let result = format!(
        "Method: {} Table: {} CSV: {}",
        form.method,
        form.table,
        form.csvpath.display()
    );
    if form.method == "insert" {
        let _ = pushdata::createtablestruct::read_csv2(
            &form.csvpath.display().to_string(),
            &form.table.to_string(),
            &form.database.to_string(),
        );
    }
    if form.method == "create" {
        let mut connection = dbconnect::database_connection(&form.database.to_string());
        let datbase = &form.database.to_string();
        let tablename = &form.table.to_string();
        let columns = getfields::read_fields(&form.csvpath.display().to_string());
        let types = getfields::read_types(&form.csvpath.display().to_string());
        let _ = tablecreate::create_table(&mut connection, &datbase, &tablename, &columns, &types);
    } else if form.method == "newdb" {
        createdatabase::create_database(&form.database.to_string());
    } else if form.method == "query" {
        let mut connection = dbconnect::database_connection(&form.database.to_string());
        let tablename = &form.table.to_string();
        //let columns=getfields::read_fields(&form.csvpath.display().to_string());
        //let types=getfields::read_types(&form.csvpath.display().to_string());
        //let queryresult= querytable::query_tables(&tablename, &mut connection,&form.csvpath.display().to_string(), &form.database.to_string());
        //println!("{:?}",queryresult);
    } else if form.method == "csv" {
        let mut connection = dbconnect::database_connection(&form.database.to_string());
        let _ = createrecord::create_session_csv(
            &mut connection,
            &form.table.to_string(),
            &form.database.to_string(),
        );
    } else {
        println!("No method selected");
    }

    println!("{}", result);
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("pages/methodsuccess.html"))
}
async fn auth(form: web::Form<Auth>) -> impl Responder {
    let result = format!("Username: {} Password: {}", form.username, form.password);

    println!("{}", result);
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("page.html"))
}
async fn query(form: web::Form<QueryData>) -> impl Responder {
    let mut connection = dbconnect::database_connection(&form.database.to_string());
    let tablename = &form.table.to_string();
    let columns = pushdata::gettablecol::get_table_col(
        &mut connection,
        &tablename,
        &form.database.to_string(),
    )
    .unwrap();
    //let types=getfields::read_types(&form.csvpath.display().to_string());
    //let queryresult= querytable::query_tables(&tablename, &mut connection,&form.whereclause.to_string(), &form.database.to_string());
    //println!("{:?}",queryresult);
    //let html=querytable::displayquery::buildhtml(queryresult, &form.database.to_string(), &form.table.to_string(), columns);
    HttpResponse::Ok()
        .content_type("text/json; charset=utf-8")
        .body("Success 200: Query Executed")
}
async fn querytojson(info: web::Path<(String, String, String, String, String)>) -> impl Responder {
    let valid = connkey::search_apikey(&info.0, &info.4);
    if valid.unwrap() == true {
        let mut connection = dbconnect::internalqueryconn();

        let database = &info.0;
        let tablename = &info.1;
        let select = &info.2;
        let whereclause = &info.3;
        //select is comma separated list of columns
        //separate select into vector
        let selectvec: Vec<&str> = select.split(',').collect();
        let select2 = selectvec.clone();

        for i in 0..selectvec.len() {
            println!("{}", selectvec[i]);
        }

        //let apikey=&info.4;

        let queryresult = querytable::query_tables(
            &tablename,
            &mut connection,
            &whereclause,
            &database,
            selectvec,
        );
        let json =
            querytable::build_json(queryresult, &database, &tablename, &mut connection, select2);

        HttpResponse::Ok()
            .content_type("text/json; charset=utf-8")
            .body(json.to_string())
    } else {
        HttpResponse::Ok()
            .content_type("text/json; charset=utf-8")
            .body("Invalid API Key")
    }
}
async fn querytableschema(body: web::Path<(String,String,String)>)->impl Responder{
    let valid = connkey::search_apikey(&body.0, &body.2);
    if valid.unwrap() == true {
        let mut connection = dbconnect::internalqueryconn();

        let database = &body.0;
        let tablename = &body.1;
        let mut select=Vec::new();
        select.push("*");
        let columnnamestmt=querytable::grab_columnnames_schema(tablename, database);
        let column=querytable::exec_map(&mut connection, &columnnamestmt.unwrap());
        let columntypestmt=querytable::grab_columntypes_schema(tablename, database);
        let columntype=querytable::exec_map(&mut connection, &columntypestmt.unwrap());
        let constraintstmt=querytable::query_constraints(tablename, database);
        let constraint=querytable::exec_map_tuple(&mut connection, &constraintstmt.unwrap());


        let json=querytable::query_table_schema(column.unwrap(), columntype.unwrap(), constraint.unwrap());
        //let queryresult = querytable::query_table_schema(
        //    &database,
        //    &tablename,
        //);
        //let json =
        //    querytable::build_jsonschema(queryresult, &database, &tablename, &mut connection);

        HttpResponse::Ok()
            .content_type("text/json; charset=utf-8")
            .body(json.to_string())
    } else {
        HttpResponse::Ok()
            .content_type("text/json; charset=utf-8")
            .body("Invalid API Key")
    }
}
async fn querydatabase(body: web::Path<(String,String,String)>)->impl Responder{
    let valid = connkey::search_apikey_admin(&body.2);
    if valid.unwrap() == true {
        let mut connection = dbconnect::internalqueryconn();

        let database = &body.0;
        //expand will be true or false
        let expand = &body.1;
        //turn into bool
        let expandbool: bool = expand.parse().unwrap();

        let mut json=serde_json::json!({});
        if expandbool==false{

        
            let mut select=Vec::new();
            select.push("*");
            let tablestmt=querytable::grab_tablenames(database);
            let tables=querytable::exec_grab_tablenames(&mut connection, &tablestmt.unwrap());
            json=querytable::json_table_names(tables.unwrap(), database);
        }
        else{
            let mut select=Vec::new();
            select.push("*");
            let tablestmt=querytable::grab_tablenames(database);
            let tablesresult=querytable::exec_grab_tablenames(&mut connection, &tablestmt.unwrap());
            let tables=tablesresult.unwrap();
            let mut storage:Vec<(&str,Vec<String>, Vec<String>, Vec<(String,String)>)>=Vec::new();
            for i in 0..tables.len(){
                let columnnamestmt=querytable::grab_columnnames_schema(&tables[i], database);
                let column=querytable::exec_map(&mut connection, &columnnamestmt.unwrap());
                let columntypestmt=querytable::grab_columntypes_schema(&tables[i], database);
                let columntype=querytable::exec_map(&mut connection, &columntypestmt.unwrap());
                let constraintsstmt=querytable::query_constraints(&tables[i], database);
                let constraints=querytable::exec_map_tuple(&mut connection, &constraintsstmt.unwrap());

                
                storage.push((&tables[i], column.unwrap(), columntype.unwrap(), constraints.unwrap()));
            }
            println!("Storage: {:?}", storage);
            json=querytable::query_database_schema(storage, database);

        }
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
async fn getcreate(form: web::Form<NewCsv>) -> impl Responder {
    let mut connection = dbconnect::database_connection(&form.database.to_string());
    let tablename = &form.table.to_string();
    let database = &form.database.to_string();
    let columns = pushdata::gettablecol::get_table_col(
        &mut connection,
        &tablename,
        &form.database.to_string(),
    )
    .unwrap();
    //let _=createrecord::create_record(&mut connection, &form.table.to_string(), &form.database.to_string(), &form.records);
    let html = createrecord::generateform::buildform(database, tablename, columns);
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}
async fn postcreate(form: web::Form<SaveNewCsv>) -> impl Responder {
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
async fn saveform(web::Form(form): web::Form<NewRecord>) -> impl Responder {
    //take form data and print it
    println!("{:?}", form);
    //let mut connection=dbconnect::database_connection(&form.database.to_string());
    //get user input from form data from create function
    //  let newrecord=NewRecord{
    //      records: form.records
    //  };
    //println!("{:?}", newrecord);
    //    newrecord.records
    let html = createrecord::generateform::formresponse(form);
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
pub struct ApiKey {
    apikey: String,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Data2 {
    columns: Vec<Column>,
}
#[derive(Debug, PartialEq, Eq)]
struct ColData {
    fields: Vec<String>,
}

#[derive(Parser)]
struct CLI {
    pattern: String,
    table: String,
    path: std::path::PathBuf,
}
#[derive(Parser, Serialize, Deserialize)]
pub struct SaveNewCsv {
    database: String,
    table: String,
    data: Vec<String>,
}
#[derive(Parser, Serialize, Deserialize)]
pub struct NewCsv {
    database: String,
    table: String,
}
#[derive(Parser, Serialize, Debug, Deserialize)]
pub struct NewRecord {
    records: Vec<String>,
}
type Column = Vec<String>;

#[derive(Parser, Serialize, Debug, Deserialize)]
pub struct Auth {
    username: String,
    password: String,
}

#[derive(Parser, Serialize, Debug, Deserialize)]
pub struct LinkDataBase {
    dbuser: String,
    dbpass: String,
    dbhost: String,
    dbport: String,
}
#[derive(Parser, Serialize, Debug, Deserialize)]
pub struct NewDataBase {
    database: String,
}

#[derive(Parser, Serialize, Debug, Deserialize)]
pub struct NewRelationShip {
    database: String,
    table1: String,
    column1: String,
    table2: String,
    column2: String,
    onupdate: String,
    ondelete: String,
}
#[derive(Parser, Serialize, Debug, Deserialize)]
pub struct CsvRequestBody {
    data: String,
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
        let new_record = NewRecord {
            records: vec!["record1".to_string(), "record2".to_string()],
        };
        assert_eq!(new_record.records.len(), 2);
    }

    #[test]
    fn test_new_database() {
        let new_database = NewDataBase {
            database: "my_database".to_string(),
        };
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
    fn test_insert_record() {
        let database = "unit_tests";
        let table = "testinsertupdatedelete";
        let data = vec![
            ("col1".to_string(), "50".to_string()),
            ("col2".to_string(), "Test Addition".to_string()),
        ];
        let data2 = vec![
            ("col1".to_string(), "50".to_string()),
            ("col2".to_string(), "Test Addition".to_string()),
        ];
        let mut body=Vec::new();
        body.push(data);
        body.push(data2);

        let mut newrecord = TableDef::new();
        //newrecord.populate(table, database);
        newrecord.table_fields.push("col1".to_string());
        newrecord.table_fields.push("col2".to_string());
        newrecord.table_types.push("int(11)".to_string());
        newrecord.table_types.push("varchar(255)".to_string());

        assert_eq!(
            newrecord.table_types,
            vec!["int(11)".to_string(), "varchar(255)".to_string()]
        );
        assert_eq!(
            newrecord.table_fields,
            vec!["col1".to_string(), "col2".to_string()]
        );
        println!("{:?}", newrecord);
        //let mut validvec: Vec<bool> = Vec::new();
        //for i in 0..newrecord.table_fields.len() {
        //    validvec.push(false);
        //    
        //    for j in 0..body.len() {
        //        if newrecord.table_fields[i] == body[j].0 {
        //            validvec[i] = true;
        //        }
        //    }
        //}
        //for i in 0..validvec.len() {
        //    assert_eq!(validvec[i], true);
        //}
        let valid = newrecord.compare_fields(&body);
        assert_eq!(valid, true);
        let insert = newrecord.insert(&body, &table, &database);
        assert_eq!(insert, String::from("INSERT INTO unit_tests.testinsertupdatedelete (col1, col2) VALUES (50, 'Test Addition'), (50, 'Test Addition')"));
    }
    #[test]
    fn test_update_record() {
        let database = "unit_tests";
        let table = "testinsertupdatedelete";
        let mut datastore=Vec::new();
        let mut data: Vec<(String, String)> = Vec::new();
        data.push(("INTERNAL_PRIMARY_KEY".to_string(), "1".to_string()));
        data.push(("col1".to_string(), "50".to_string()));
        data.push(("col2".to_string(), "Changed".to_string()));
        datastore.push(data);

        let update = update::updaterecord(database, table, datastore);
        //assert_eq!(update.unwrap(), String::from("Success"));
        assert_eq!(update[0], String::from("UPDATE unit_tests.testinsertupdatedelete SET col1= \"50\", col2= \"Changed\" WHERE INTERNAL_PRIMARY_KEY=1"));
    }
    #[test]
    fn test_delete_record() {
        let database = "unit_tests";
        let table = "testinsertupdatedelete";
        let mut data: Vec<(String, String)> = Vec::new();
        data.push(("1".to_string(), "1".to_string()));
        data.push(("2".to_string(), "2".to_string()));
        let statement = delete::deleterecord(database, table, data);
        assert_eq!(statement.unwrap(), String::from("DELETE FROM unit_tests.testinsertupdatedelete WHERE INTERNAL_PRIMARY_KEY in( 1, 2)"));
    }
    #[test]
    fn test_drop_table() {
        let database = "unit_tests";
        let table = "testinsertupdatedelete";
        let statement = delete::droptable(database, table);
        assert_eq!(
            statement.unwrap(),
            String::from("DROP TABLE unit_tests.testinsertupdatedelete")
        );
    }
    #[test]
    fn test_drop_backup() {
        let database = "unit_tests";
        let table = "testinsertupdatedelete";
        let statement = delete::generate_backup(database, table);
        assert_eq!(statement.unwrap(), String::from("SELECT * INTO OUTFILE '/tmp/unit_tests_testinsertupdatedelete.csv' FIELDS TERMINATED BY ',' OPTIONALLY ENCLOSED BY '\"' LINES TERMINATED BY '\n' FROM unit_tests.testinsertupdatedelete"));
    }
    #[test]
    fn test_grabcolumns() {
        let database = "unit_tests";
        let table = "testinsertupdatedelete";
        let mut select = Vec::new();
        select.push("col1");
        select.push("col2");
        select.push("col10");
        let statement = querytable::grab_columnnames(table, database, select);
        assert_eq!(statement.unwrap(), String::from("SELECT COLUMN_NAME FROM INFORMATION_SCHEMA.COLUMNS WHERE TABLE_SCHEMA = 'unit_tests' AND TABLE_NAME = 'testinsertupdatedelete'And COLUMN_NAME != 'INTERNAL_PRIMARY_KEY'And COLUMN_NAME != 'GPS_ID'And COLUMN_NAME != 'X_COORD'And COLUMN_NAME != 'Y_COORD'And COLUMN_NAME != 'Attachment'And COLUMN_NAME in ( 'col1', 'col2', 'col10')"));
    }
    #[test]
    fn test_grabcolumtypes() {
        let database = "unit_tests";
        let table = "testinsertupdatedelete";
        let mut select = Vec::new();
        select.push("col1");
        select.push("col2");
        select.push("col10");
        let statement = querytable::grab_columntypes(table, database);
        assert_eq!(statement.unwrap(), String::from("SELECT COLUMN_TYPE FROM INFORMATION_SCHEMA.COLUMNS WHERE TABLE_SCHEMA = 'unit_tests' AND TABLE_NAME = 'testinsertupdatedelete'And COLUMN_NAME != 'INTERNAL_PRIMARY_KEY'And COLUMN_NAME != 'GPS_ID'And COLUMN_NAME != 'X_COORD'And COLUMN_NAME != 'Y_COORD'And COLUMN_NAME != 'Attachment'"));
    }
    #[test]
    fn test_protected_terms() {
        let mut terms=Vec::new();
        let term=String::from("INTERNAL_PRIMARY_KEY");
        let term2=String::from("CONDITION");
        let term3=String::from("INT");
        let term4=String::from("VARCHAR");
        let term5=String::from("CONDITION_RATING");

        terms.push(term);
        terms.push(term2);
        terms.push(term3);
        terms.push(term4);

        for term in terms.iter(){
            let valid=tablecreate::validate_unprotected_term(term);
            assert_eq!(valid.0, false);
        }

        let valid=tablecreate::validate_unprotected_term(&term5);
        assert_eq!(valid.0, true);


        
    }
//    #[test]
//    fn test_create_table() {
//        let database = "unit_tests";
//        let table = "testinsertupdatedelete";
//        //use first column of tuple to get column names
//        //use second column of tuple to get column types
//        let body:web::Json<Value>=[("columns", "[{\"col1\":\"condition_rtg\",\"col2\":\"color\",\"col3\":\"part_number\",\"col4\":\"runtime\"}]"), ("types", "[{\"col1\":\"INT(11)\",\"col2\":\"VARCHAR(255)\",\"col3\":\"VARCHAR(255)\",\"col4\":\"INT(11)\"}]")];
//        
//        let body=body.into_inner();
//
//
//        let mut json=Vec::new();
//        json.push(body);
//
//        //
//        let parsed=tablecreate::parse_json(body);
//        let statement = tablecreate::create_table_web(database, table, &parsed.0, &parsed.1);
//        assert_eq!(
//            statement.unwrap(),
//            String::from("CREATE TABLE unit_tests.testinsertupdatedelete (INTERNAL_PRIMARY_KEY INT NOT NULL AUTO_INCREMENT PRIMARY KEY, condition_rtg INT(11), color VARCHAR(255), part_number VARCHAR(255), runtime INT(11))")
//        );
//    }
//} 
//
}
