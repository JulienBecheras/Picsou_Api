use diesel::prelude::*;
use chrono::NaiveDateTime;
use rocket::serde::Deserialize;
use serde::Serialize;

#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = crate::schema::friend_requests)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct FriendRequest {
    pub id: i32,
    pub from_user_id: i32,
    pub to_user_id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

// Struct pour les INSERTS sans les champs auto-générés
#[derive(Insertable, Deserialize, Clone)]
#[diesel(table_name = crate::schema::friend_requests)]
pub struct InsertableFriendRequest {
    pub from_user_id: i32,
    pub to_user_id: i32,
}