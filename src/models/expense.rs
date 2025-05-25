use diesel::prelude::*;
use chrono::NaiveDateTime;
use rocket::serde::Deserialize;
use serde::Serialize;

#[derive(Queryable, Selectable, Serialize, Clone, AsChangeset, Deserialize)]
#[diesel(table_name = crate::schema::expenses)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub montant: f64,
    pub stock_parts: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

// Struct pour les INSERTS sans les champs auto-générés
#[derive(Insertable, Deserialize, Clone)]
#[diesel(table_name = crate::schema::expenses)]
pub struct InsertableUser {
    pub name: String,
    pub description: String,
    pub montant: f64,
    pub stock_parts: i32,
}
