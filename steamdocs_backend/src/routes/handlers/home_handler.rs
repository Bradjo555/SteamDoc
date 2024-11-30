use actix_web::{get, web, Responder};
use sea_orm::{ConnectionTrait, Statement};

use crate::utils::{api_response, app_state::AppState};

#[get("/")]
pub async fn greet() -> impl Responder {
    api_response::ApiResponse::new(200, "<h1> Hello, world! </h1>".to_string())
}

#[get("/test")]
pub async fn test_database(app_state: web::Data<AppState>) -> impl Responder {
    let _res = app_state
        .db
        .query_all(Statement::from_string(
            sea_orm::DatabaseBackend::Postgres,
            "Select * from user;",
        ))
        .await
        .unwrap();
    api_response::ApiResponse::new(200, "Testing database".to_string())
}
