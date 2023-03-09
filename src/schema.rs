// @generated automatically by Diesel CLI.

diesel::table! {
    song (id) {
        id -> Varchar,
        link -> Varchar,
        user_email -> Varchar,
    }
}

diesel::table! {
    user (email) {
        id -> Varchar,
        email -> Varchar,
    }
}

diesel::joinable!(song -> user (user_email));

diesel::allow_tables_to_appear_in_same_query!(
    song,
    user,
);
