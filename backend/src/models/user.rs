use serde::{Serialize, Deserialize};
use diesel::prelude::*;
use crate::schema::users;

#[derive(Queryable, Insertable, Serialize, Deserialize)]
#[table_name = "users"]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password_hash: String,
    pub user_type: String, // 'adopter' or 'shelter'
    pub legal_accepted: bool,
}

#[derive(Deserialize)]
pub struct NewUser {
    pub name: String,
    pub email: String,
    pub password: String,
    pub user_type: String,
    pub legal_accepted: bool,
}
