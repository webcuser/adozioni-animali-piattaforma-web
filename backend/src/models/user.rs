use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewUser {
    pub name: String,
    pub surname: Option<String>,
    pub email: String,
    pub password: String,
    pub phone: Option<String>,
    pub city: Option<String>,
    pub association_name: Option<String>,
    pub contact_person: Option<String>,
    pub address: Option<String>,
    pub description: Option<String>,
    pub verification_documents: Option<String>,
    pub accepted_terms: bool,
}
