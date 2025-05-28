use chrono::NaiveDateTime;
use diesel::{AsChangeset, Insertable, Queryable, Selectable};
use rocket::serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Serialize, Clone, AsChangeset, Deserialize)]
#[diesel(table_name = crate::schema::refunds)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Refund {
    pub id: i32,
    pub amount: f64,
    pub status: String,
    pub contributors_id: i32,
    pub participants_id: i32,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}


// Struct pour les INSERTS sans les champs auto-générés
#[derive(Insertable, Deserialize, Clone)]
#[diesel(table_name = crate::schema::refunds)]
pub struct InsertableRefund {
    pub amount: f64,
    pub status: String,
    pub contributors_id: i32,
    pub participants_id: i32,
    pub created_at: Option<NaiveDateTime>,
}