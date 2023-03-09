use rocket::serde::json::{Value,Json};
use rust_backend_bin::{models::models::{UserInputUser, UserInputSong}};

use crate::services;

#[get("/user")]
pub fn get_user() -> Value {
    services::controller::get_user()
}

#[post("/user", format = "json", data = "<user_info>")]
pub fn create_user(user_info: Json<UserInputUser>) -> Value {
    services::controller::add_user(&user_info.email)
}

#[post("/song", format = "json", data = "<song_info>")]
pub fn create_song(song_info: Json<UserInputSong>) -> Value {
    services::controller::add_song(&song_info)
}
