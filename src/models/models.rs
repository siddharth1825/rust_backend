use diesel::{prelude::*};
use rocket::serde::{Serialize,Deserialize};

use crate::schema::{song,user};

#[derive(Queryable,Serialize,Deserialize)]
pub struct User {
    pub id: String,
    pub email: String,
}

#[derive(Insertable,Serialize,AsChangeset)]
#[diesel(table_name = user)]
pub struct NewUser<'a>{
    pub id: &'a str,
    pub email: &'a str,
}

#[derive(Deserialize)]
pub struct UserInputUser {
    pub email: String,
}

#[derive(Deserialize,Serialize)]
pub struct UserInputUpdateUser {
    pub email: Option<String>,
}

#[derive(Queryable,Serialize,Deserialize)]
pub struct Song {
    pub id: String,
    pub link: String,
    pub user_email: String,
}

#[derive(Insertable,Serialize,AsChangeset)]
#[diesel(table_name = song)]
pub struct NewSong<'a> {
    pub id: &'a str,
    pub link: &'a str,
    pub user_email: &'a str,
}

#[derive(Deserialize)]
pub struct UserInputSong {
    pub link: String,
    pub user_email: String,
}











