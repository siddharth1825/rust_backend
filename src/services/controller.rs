use diesel::prelude::*;
use rust_backend_bin::{
    models::models::{NewUser,NewSong,User,Song,UserInputSong},
    *,
};

use rocket::serde::json::{json,Value};

pub fn get_user() -> Value {
    use rust_backend_bin::schema::user::dsl::*;
    let connection = &mut establish_connection();
    let result: Vec<User> = user.load::<User>(connection).expect("error loading users");
    json!(result)
}

pub fn add_user(email: &str) -> Value {
    use rust_backend_bin::schema::user;
    let id = uuid::Uuid::new_v4().to_string();
    let connection = &mut establish_connection();
    let new_user: NewUser = NewUser { id: &id, email };
    let created_user: User = diesel::insert_into(user::table)
        .values(&new_user)
        .get_result::<User>(connection)
        .expect("error adding new role");

    json!(created_user)
}

pub fn add_song(song_details: &UserInputSong) -> Value {
    use rust_backend_bin::schema::song;
    let song_id = uuid::Uuid::new_v4().to_string();
    let connection = &mut establish_connection();
    let new_song: NewSong = NewSong { 
        id: &song_id, 
        link: &song_details.link, 
        user_email: &song_details.user_email, 
    };

    let created_song: Song = diesel::insert_into(song::table)
        .values(&new_song)
        .get_result::<Song>(connection)
        .expect("error adding song");

    json!(created_song)
}








