use actix_web::{dev::Payload, error::ErrorUnauthorized, web, Error, FromRequest, HttpRequest, HttpResponse, Responder};
use diesel::prelude::*;
use diesel::sql_types::{Int4, Text};
use diesel::QueryableByName;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::future::{ready, Ready};

use crate::dbconnect;
use crate::PooledConn;

#[derive(Clone)]
pub struct AppState {
    pub jwt_secret: String,
}

impl AppState {
    pub fn from_env() -> Self {
        let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "dev-secret-change-me".to_string());
        AppState { jwt_secret: secret }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i32,
    pub exp: usize,
    pub role: String,
    pub schemas: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Authenticated {
    pub user_id: i32,
    pub role: String,
    pub schemas: Vec<String>,
}

impl Authenticated {
    pub fn can_access_schema(&self, schema: &str) -> bool {
        if self.role == "admin" {
            return true;
        }

        if self.schemas.is_empty() {
            return false;
        }

        self.schemas.iter().any(|s| s == schema)
    }
}

impl FromRequest for Authenticated {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let header = req
            .headers()
            .get("Authorization")
            .and_then(|h| h.to_str().ok())
            .unwrap_or("")
            .to_string();

        if !header.starts_with("Bearer ") {
            return ready(Err(ErrorUnauthorized("Missing or invalid Authorization header")));
        }

        let token = header.trim_start_matches("Bearer ");

        let state = match req.app_data::<web::Data<AppState>>() {
            Some(data) => data.clone(),
            None => {
                return ready(Err(ErrorUnauthorized("Auth state not configured")));
            }
        };

        let decoding_key = DecodingKey::from_secret(state.jwt_secret.as_bytes());
        let validation = Validation::new(Algorithm::HS256);

        match decode::<Claims>(token, &decoding_key, &validation) {
            Ok(data) => {
                let claims = data.claims;
                let auth = Authenticated {
                    user_id: claims.sub,
                    role: claims.role,
                    schemas: claims.schemas,
                };
                ready(Ok(auth))
            }
            Err(_) => ready(Err(ErrorUnauthorized("Invalid token"))),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
    pub allowed_schemas: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(QueryableByName)]
struct UserRow {
    #[diesel(sql_type = Int4)]
    id: i32,
    #[diesel(sql_type = Text)]
    email: String,
    #[diesel(sql_type = Text)]
    password_hash: String,
    #[diesel(sql_type = Text)]
    role: String,
    #[diesel(sql_type = Text)]
    allowed_schemas: String,
}

#[derive(QueryableByName)]
struct UserCount {
    #[diesel(sql_type = Int4)]
    count: i32,
}

fn get_connection() -> PooledConn {
    dbconnect::internalqueryconn()
}

fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
    use argon2::password_hash::{rand_core::OsRng, PasswordHasher, SaltString};
    use argon2::{Algorithm, Argon2, Params, Version};

    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, Params::default());
    let hash = argon2.hash_password(password.as_bytes(), &salt)?.to_string();
    Ok(hash)
}

fn verify_password(password: &str, password_hash: &str) -> bool {
    use argon2::password_hash::{PasswordHash, PasswordVerifier};
    use argon2::Argon2;

    let parsed_hash = PasswordHash::new(password_hash);
    if parsed_hash.is_err() {
        return false;
    }
    let parsed_hash = parsed_hash.unwrap();
    Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok()
}

fn create_token(user: &UserRow, schemas: Vec<String>, secret: &str) -> Result<String, jsonwebtoken::errors::Error> {
    use chrono::Utc;

    let exp = (Utc::now().timestamp() + 3600) as usize; // 1 hour expiry
    let claims = Claims {
        sub: user.id,
        exp,
        role: user.role.clone(),
        schemas,
    };

    let header = Header::new(Algorithm::HS256);
    encode(&header, &claims, &EncodingKey::from_secret(secret.as_bytes()))
}

pub async fn register(
    body: web::Json<RegisterRequest>,
) -> impl Responder {
    let body = body.into_inner();

    if body.email.trim().is_empty() || body.password.len() < 8 {
        return HttpResponse::BadRequest()
            .content_type("application/json; charset=utf-8")
            .body("{\"error\":\"Invalid email or password too short\"}");
    }

    let schemas_vec = body.allowed_schemas.unwrap_or_else(|| vec![]);
    let allowed_schemas = if schemas_vec.is_empty() {
        String::new()
    } else {
        schemas_vec.join(",")
    };

    let password_hash = match hash_password(&body.password) {
        Ok(h) => h,
        Err(_) => {
            return HttpResponse::InternalServerError()
                .content_type("application/json; charset=utf-8")
                .body("{\"error\":\"Failed to hash password\"}");
        }
    };

    let mut conn = get_connection();

    // Determine role. By default the first registered user becomes admin
    // (for easy bootstrap), but this behavior can be disabled by setting
    // AUTH_BOOTSTRAP_MODE to any value other than "first-user-admin".
    let bootstrap_mode = std::env::var("AUTH_BOOTSTRAP_MODE")
        .unwrap_or_else(|_| "first-user-admin".to_string());

    let role = if bootstrap_mode == "first-user-admin" {
        let count_row = diesel::sql_query("SELECT COUNT(*)::INT as count FROM \"Auth\".users")
            .get_result::<UserCount>(&mut conn)
            .unwrap_or(UserCount { count: 0 });

        if count_row.count == 0 {
            "admin".to_string()
        } else {
            "user".to_string()
        }
    } else {
        "user".to_string()
    };

    let query =
        "INSERT INTO \"Auth\".users (email, password_hash, role, allowed_schemas) VALUES ($1, $2, $3, $4)";

    let result = diesel::sql_query(query)
        .bind::<Text, _>(&body.email)
        .bind::<Text, _>(&password_hash)
        .bind::<Text, _>(&role)
        .bind::<Text, _>(&allowed_schemas)
        .execute(&mut conn);

    match result {
        Ok(_) => HttpResponse::Ok()
            .content_type("application/json; charset=utf-8")
            .body("{\"status\":\"ok\"}"),
        Err(e) => {
            let msg = format!("{{\"error\":\"{}\"}}", e.to_string());
            HttpResponse::InternalServerError()
                .content_type("application/json; charset=utf-8")
                .body(msg)
        }
    }
}

pub async fn login(
    state: web::Data<AppState>,
    body: web::Json<LoginRequest>,
) -> impl Responder {
    let body = body.into_inner();

    let mut conn = get_connection();

    let query =
        "SELECT id, email, password_hash, role, allowed_schemas FROM \"Auth\".users WHERE email = $1";

    let result: Result<UserRow, _> = diesel::sql_query(query)
        .bind::<Text, _>(&body.email)
        .get_result(&mut conn);

    let user = match result {
        Ok(u) => u,
        Err(_) => {
            return HttpResponse::Unauthorized()
                .content_type("application/json; charset=utf-8")
                .body("{\"error\":\"Invalid credentials\"}");
        }
    };

    if !verify_password(&body.password, &user.password_hash) {
        return HttpResponse::Unauthorized()
            .content_type("application/json; charset=utf-8")
            .body("{\"error\":\"Invalid credentials\"}");
    }

    let schemas: Vec<String> = if user.allowed_schemas.trim().is_empty() {
        Vec::new()
    } else {
        user.allowed_schemas
            .split(',')
            .map(|s| s.trim().to_string())
            .collect()
    };

    let token = match create_token(&user, schemas, &state.jwt_secret) {
        Ok(t) => t,
        Err(_) => {
            return HttpResponse::InternalServerError()
                .content_type("application/json; charset=utf-8")
                .body("{\"error\":\"Failed to create token\"}");
        }
    };

    let resp = serde_json::json!({
        "status": "ok",
        "token": token,
    });

    HttpResponse::Ok()
        .content_type("application/json; charset=utf-8")
        .body(resp.to_string())
}

#[derive(Serialize)]
struct PublicUser {
    id: i32,
    email: String,
    role: String,
    allowed_schemas: String,
}

#[derive(Deserialize)]
pub struct UpdateUserSchemasRequest {
    pub email: String,
    pub allowed_schemas: Vec<String>,
}

pub async fn list_users(auth: Authenticated) -> impl Responder {
    if auth.role != "admin" {
        return HttpResponse::Forbidden()
            .content_type("application/json; charset=utf-8")
            .body("{\"error\":\"Admin role required\"}");
    }

    let mut conn = get_connection();

    let query = "SELECT id, email, role, allowed_schemas FROM \"Auth\".users";

    let result: Result<Vec<UserRow>, _> = diesel::sql_query(query).load(&mut conn);

    match result {
        Ok(rows) => {
            let users: Vec<PublicUser> = rows
                .into_iter()
                .map(|u| PublicUser {
                    id: u.id,
                    email: u.email,
                    role: u.role,
                    allowed_schemas: u.allowed_schemas,
                })
                .collect();

            let resp = serde_json::to_string(&users).unwrap_or_else(|_| "[]".to_string());
            HttpResponse::Ok()
                .content_type("application/json; charset=utf-8")
                .body(resp)
        }
        Err(e) => {
            let msg = format!("{{\"error\":\"{}\"}}", e.to_string());
            HttpResponse::InternalServerError()
                .content_type("application/json; charset=utf-8")
                .body(msg)
        }
    }
}

pub async fn update_user_schemas(
    auth: Authenticated,
    body: web::Json<UpdateUserSchemasRequest>,
) -> impl Responder {
    if auth.role != "admin" {
        return HttpResponse::Forbidden()
            .content_type("application/json; charset=utf-8")
            .body("{\"error\":\"Admin role required\"}");
    }

    let body = body.into_inner();
    let allowed_schemas = if body.allowed_schemas.is_empty() {
        String::new()
    } else {
        body.allowed_schemas.join(",")
    };

    let mut conn = get_connection();

    let query =
        "UPDATE \"Auth\".users SET allowed_schemas = $1 WHERE email = $2";

    let result = diesel::sql_query(query)
        .bind::<Text, _>(&allowed_schemas)
        .bind::<Text, _>(&body.email)
        .execute(&mut conn);

    match result {
        Ok(_) => HttpResponse::Ok()
            .content_type("application/json; charset=utf-8")
            .body("{\"status\":\"ok\"}"),
        Err(e) => {
            let msg = format!("{{\"error\":\"{}\"}}", e.to_string());
            HttpResponse::InternalServerError()
                .content_type("application/json; charset=utf-8")
                .body(msg)
        }
    }
}
