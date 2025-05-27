use diesel::prelude::*;
use chrono::NaiveDateTime;
use rocket::serde::Deserialize;
use serde::Serialize;
use crate::models::user::User;

#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = crate::schema::friends)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Friend {
    pub id: i32,
    pub user1_id: i32,
    pub user2_id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
#[derive(Clone, Deserialize, Serialize)]
pub struct DetailedFriend {
    pub id: i32,
    pub user1: User,
    pub user2: User,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

// Struct pour les INSERTS sans les champs auto-générés
#[derive(Insertable, Deserialize, Clone)]
#[diesel(table_name = crate::schema::friends)]
pub struct InsertableFriend {
    pub user1_id: i32,
    pub user2_id: i32,
}