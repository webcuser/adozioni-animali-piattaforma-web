use jsonwebtoken::{encode, Header, EncodingKey};
use serde::{Deserialize, Serialize};
use bcrypt::{hash, verify};
use crate::models::user::{User, NewUser};
use crate::db::DbPool;
use actix_web::{web, HttpResponse, Result};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

pub async fn register_user(pool: web::Data<DbPool>, new_user: web::Json<NewUser>) -> Result<HttpResponse> {
    let conn = pool.get().expect("Couldn't get db connection from pool");
    let hashed_password = hash(&new_user.password, 4).unwrap();
    let user = NewUser {
        password: hashed_password,
        ..new_user.into_inner()
    };
    // Save user to the database
    // Omitted: Database logic to insert user
    Ok(HttpResponse::Created().finish())
}

pub async fn login_user(pool: web::Data<DbPool>, login_info: web::Json<User>) -> Result<HttpResponse> {
    let conn = pool.get().expect("Couldn't get db connection from pool");
    // Omitted: Fetch user from database and verify password
    let is_valid = verify(&login_info.password, "hashed_password_from_db").unwrap();
    if is_valid {
        let claims = Claims {
            sub: login_info.email.clone(),
            exp: 10000000000,
        };
        let token = encode(&Header::default(), &claims, &EncodingKey::from_secret("secret".as_ref())).unwrap();
        Ok(HttpResponse::Ok().json(token))
    } else {
        Ok(HttpResponse::Unauthorized().finish())
    }
}
