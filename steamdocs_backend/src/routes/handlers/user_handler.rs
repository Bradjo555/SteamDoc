use actix_web::get;
use actix_web::{post, web};
use sea_orm::{EntityTrait, Set};
use sea_orm::ActiveModelTrait;
use serde::{Deserialize, Serialize};
use sha256::digest;

use crate::utils::{api_response::{self, ApiResponse}, app_state};

#[derive(Serialize, Deserialize)]
struct NewUserModel {
    name: String,
    email: String,
    password: String,
}

#[get("/{user_id}")]
pub async fn get_user_by_id(app_state: web::Data<app_state::AppState> ,path: web::Path<i32>) -> Result<ApiResponse, ApiResponse> {
    let user_id   = path.into_inner();
    
    match entity::user::Entity::find_by_id(user_id)
    .one(&app_state.db)
    .await
    {
        Ok(Some(user)) => {
            Ok(ApiResponse::new(200, format!("User: {:?} found", user.name.to_string())))
        }
        Ok(None) => {
            Ok(ApiResponse::new(404, format!("User with id {} not found", user_id)))
        }
        Err(err) => {
            Err(ApiResponse::new(500, err.to_string()))
        }
    }
}

#[post("/")]
pub async fn create_user(
    app_state: web::Data<app_state::AppState>,
    register_json: web::Json<NewUserModel>) -> Result<ApiResponse, ApiResponse> {
    let user_model = entity::user::ActiveModel {
        name: Set(register_json.name.clone()),
        email: Set(register_json.email.clone()),
        password: Set(digest(&register_json.password)),
        ..Default::default()
    }.insert(&app_state.db).await
    .map_err(|err| ApiResponse::new(500, err.to_string()))?;

    Ok(api_response::ApiResponse::new(200, format!("user {:?} created", user_model.name)))
}