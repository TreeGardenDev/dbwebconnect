use actix_web::{test, web, App};

// Import the same modules main.rs uses
use dbwebconnect::auth;
use dbwebconnect::createdatabase;
use dbwebconnect::createrecord;
use dbwebconnect::createrelationship;
use dbwebconnect::dbconnect;
use dbwebconnect::delete;
use dbwebconnect::getfields;
use dbwebconnect::initconnect;
use dbwebconnect::insertrecords;
use dbwebconnect::pushdata;
use dbwebconnect::querytable;
use dbwebconnect::relationships;
use dbwebconnect::tablecreate;
use dbwebconnect::update;

// NOTE: These are integration-style tests that expect:
// - A running Postgres instance reachable via DATABASE_URL
// - The initapi.sql migrations applied (so Auth, Relationships schemas exist)
// You can run them with `cargo test` once those prerequisites are satisfied.

fn app_factory() -> App<impl actix_web::dev::ServiceFactory<
    actix_web::dev::ServiceRequest,
    Config = (),
    Response = actix_web::dev::ServiceResponse,
    Error = actix_web::Error,
    InitError = (),
>> {
    App::new()
        .app_data(web::Data::new(auth::AppState::from_env()))
        .service(
            web::resource("/")
                .route(web::get().to(crate::getinitializeconnect))
                .route(web::post().to(crate::postinitializeconnect)),
        )
        .route("/auth/register", web::post().to(auth::register))
        .route("/auth/login", web::post().to(auth::login))
        .route("/admin/users", web::get().to(auth::list_users))
        .route(
            "/admin/users/schemas",
            web::post().to(auth::update_user_schemas),
        )
        .route("/health", web::get().to(crate::health))
        .route("/getkey/{database}&apikey={apikey}", web::get().to(crate::getkey))
        .route(
            "/createtable/{database}&table={table}&gps={gps}&apikey={apikey}",
            web::post().to(crate::createtableweb),
        )
        .route(
            "/droptable/{database}&table={table}&apikey={apikey}",
            web::post().to(crate::droptableweb),
        )
        .route(
            "/createdatabase/{database}&apikey={apikey}",
            web::post().to(crate::createnewdbweb),
        )
        .route(
            "/query/{database}&table={table}&select={select}&where={where}&expand={expand}&apikey={api}",
            web::get().to(crate::querytojson),
        )
        .route(
            "/querytableschema/{database}&table={table}&apikey={api}",
            web::get().to(crate::querytableschema),
        )
        .route(
            "/querydatabase/{database}&expand={expand}&apikey={api}",
            web::get().to(crate::querydatabase),
        )
        .route(
            "/queryrelationship/{database}&relationship={relationship}&apikey={api}",
            web::get().to(crate::queryrelationship),
        )
        .route(
            "/queryall/{database}&table={table}&depth={depth}&apikey={api}",
            web::get().to(crate::queryall),
        )
        .route(
            "/insert/{database}&table={table}&apikey={api}",
            web::post().to(crate::dbinsert),
        )
        .route(
            "/insertattachment/{database}&table={table}&apikey={api}",
            web::post().to(crate::dbinsertattachment),
        )
        .route(
            "/retrieveattachment/{database}&table={table}&id={id}&apikey={api}",
            web::get().to(crate::retrieveattachment),
        )
        .route(
            "/updaterecord/{database}&table={table}&apikey={api}",
            web::post().to(crate::dbupdaterecord),
        )
        .route(
            "/relationship/{database}&apikey={api}",
            web::post().to(crate::createrelationshipweb),
        )
        .route(
            "relateparent/{database}&parent_table={parent_table}&child_table={child_table}&relationship_name={relationship_name}&apikey={api}",
            web::post().to(crate::createrelationshipparentweb),
        )
        .route(
            "/deleterecord/{database}&table={table}&apikey={api}",
            web::post().to(crate::deleterecord),
        )
}

#[actix_web::test]
async fn health_smoke() {
    let app = test::init_service(app_factory()).await;
    let req = test::TestRequest::get().uri("/health").to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
}

// High-level flow: register + login + simple schema lifecycle
#[actix_web::test]
async fn auth_and_schema_lifecycle_smoke() {
    let app = test::init_service(app_factory()).await;

    // 1) Register first user (becomes admin)
    let register_body = serde_json::json!({
        "email": "admin@test.local",
        "password": "Password123!"
    });
    let req = test::TestRequest::post()
        .uri("/auth/register")
        .set_json(&register_body)
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    // 2) Login to get JWT
    let login_body = serde_json::json!({
        "email": "admin@test.local",
        "password": "Password123!"
    });
    let req = test::TestRequest::post()
        .uri("/auth/login")
        .set_json(&login_body)
        .to_request();
    let resp = test::call_and_read_body(&app, req).await;
    let json: serde_json::Value = serde_json::from_slice(&resp).unwrap();
    let token = json["token"].as_str().unwrap().to_string();

    // 3) Create logical schema
    let req = test::TestRequest::post()
        .uri("/createdatabase/test_schema&apikey=ignored")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    // 4) Create simple table in that schema
    let body = serde_json::json!({
        "columns": "[{\"name:id\"},{\"name:name\"}]",
        "types":   "[{\"type:INT\"},{\"type:VARCHAR(255)\"}]"
    });
    let req = test::TestRequest::post()
        .uri("/createtable/test_schema&table=test_table&gps=false&apikey=ignored")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(&body)
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
}

// Skeletons for additional scenarios; fill out as needed.
// - attachment_roundtrip
// - relationship_roundtrip
// - queryall_depth_control
