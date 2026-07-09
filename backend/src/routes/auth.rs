use actix_web::{post, web, HttpResponse, Responder};
use crate::models::user::{User, NewUser};
use crate::auth::{hash_password, verify_password, create_jwt};
use diesel::prelude::*;
use crate::schema::users::dsl::*;
use crate::DbPool;

#[post("/register")]
async fn register_user(pool: web::Data<DbPool>, form: web::Json<NewUser>) -> impl Responder {
    if !form.legal_accepted {
        return HttpResponse::BadRequest().body("Legal terms must be accepted.");
    }

    let conn = pool.get().expect("couldn't get db connection from pool");
    let password_hash = match hash_password(&form.password) {
        Ok(hash) => hash,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    let new_user = User {
        id: 0, // Will be set by the database
        name: form.name.clone(),
        email: form.email.clone(),
        password_hash,
        user_type: form.user_type.clone(),
        legal_accepted: form.legal_accepted,
    };

    diesel::insert_into(users)
        .values(&new_user)
        .execute(&conn)
        .expect("Error saving new user");

    HttpResponse::Ok().body("User registered successfully")
}

#[post("/login")]
async fn login_user(pool: web::Data<DbPool>, form: web::Json<NewUser>) -> impl Responder {
    let conn = pool.get().expect("couldn't get db connection from pool");
    let user = users
        .filter(email.eq(&form.email))
        .first::<User>(&conn)
        .optional()
        .expect("Error loading user");

    if let Some(user) = user {
        if verify_password(&form.password, &user.password_hash).unwrap_or(false) {
            match create_jwt(&user.id.to_string()) {
                Ok(token) => return HttpResponse::Ok().json(token),
                Err(_) => return HttpResponse::InternalServerError().finish(),
            }
        }
    }

    HttpResponse::Unauthorized().body("Invalid credentials")
}
