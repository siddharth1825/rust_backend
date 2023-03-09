use rocket::serde::json::{Value};

use crate::services;

#[get("/")]
pub fn index() -> Value {
    services::index::home()
}
