use diesel::prelude::*;
use rust_backend_bin::{
    models::models::{NewUser,NewSong,User,Song,UserInputSong},
    *,
};
use std::collections::HashMap;

use rocket::serde::json::{json,Value};

pub fn get_user() -> Value {
    use rust_backend_bin::schema::{user, song};
    let mut connection = establish_connection();
    let results = user::table
        .left_outer_join(song::table)
        .select((user::id, user::email, song::id.nullable(), song::link.nullable()))
        .load::<(String, String, Option<String>, Option<String>)>(&mut connection)
        .expect("error loading users");

    let mut users: HashMap<String, HashMap<String, Value>> = HashMap::new();
    for (user_id, user_email, song_id, song_title) in results {
        let user = users.entry(user_id).or_insert_with(|| {
            let mut map = HashMap::new();
            map.insert("email".to_string(), json!(user_email));
            map.insert("songs".to_string(), json!(Vec::<Value>::new()));
            map
        });

        if let Some(song_id) = song_id {
            let song = json!({
                "id": song_id,
                "title": song_title.unwrap_or_else(|| "".to_string())
            });
            let songs = user.get_mut("songs").unwrap().as_array_mut().unwrap();
            songs.push(song);
        }
    }

    let users = users.values().cloned().collect::<Vec<HashMap<String, Value>>>();
    json!(users)
}

pub fn get_song() -> Value {
    use rust_backend_bin::schema::{song::dsl::*, user::dsl::*};
    let connection = &mut establish_connection();
    let results = song
        .inner_join(user)
        .load::<(Song, User)>(connection)
        .expect("error loading songs");

    json!(results)
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

pub fn get_one_user(user_email: &str) -> Value {
    use rust_backend_bin::schema::{user, song};
    let connection = &mut establish_connection();

    let result = user::table
        .filter(user::email.eq(user_email))
        .left_outer_join(song::table)
        .select((user::id, user::email, song::id.nullable(), song::link.nullable()))
        .load::<(String, String, Option<String>, Option<String>)>(connection)
        .expect("error loading user");

    let mut user_data: HashMap<String, Value> = HashMap::new();
    let songs: Vec<Value> = result
        .into_iter()
        .filter_map(|(user_id, _, song_id, song_link)| {
            if let Some(song_id) = song_id {
                Some(json!({
                    "id": song_id,
                    "link": song_link.unwrap_or_else(|| "".to_string())
                }))
            } else {
                None
            }
        })
        .collect();

    user_data.insert("email".to_string(), json!(user_email));
    user_data.insert("songs".to_string(), json!(songs));

    json!(user_data)
}












