use diesel::prelude::*;
use chrono::NaiveDateTime;
use rocket::serde::Deserialize;
use serde::Serialize;

#[derive(Queryable, Selectable, Serialize, Clone, AsChangeset, Deserialize)]
#[diesel(table_name = crate::schema::contributors)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Contributor {
    pub id: i32,
    pub amount_contributed: f64,
    pub groups_users_id: i32,
    pub expenses_id: i32,
}

// Struct pour les INSERTS sans les champs auto-générés
#[derive(Insertable, Deserialize, Clone)]
#[diesel(table_name = crate::schema::contributors)]
pub struct InsertableContributor {
    pub amount_contributed: f64,
    pub groups_users_id: i32,
    pub expenses_id: i32,
}