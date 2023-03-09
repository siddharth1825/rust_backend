#[macro_use] extern crate rocket;
use rocket::{serde::{json::{json,Value}}};

mod routes;
mod services;

use routes::index::index;
use routes::routes::{get_user,create_user,create_song};

#[catch(404)]
fn not_found()->Value{
    json!({
        "status":"error",
        "reason":"resource not found"
    })
}

#[catch(500)]
fn server_error()->Value{
    json!({
        "status":"error",
        "reason":"something went wrong please try again"
    })
}

#[launch]
fn rocket()->_{
    rocket::build().mount("/",routes![index,get_user,create_user,create_song])
        .register("/", catchers![not_found,server_error])
}
