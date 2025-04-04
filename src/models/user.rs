use diesel::prelude::*;
use chrono::NaiveDateTime;
use rocket::serde::Deserialize;
use serde::Serialize;

#[derive(Queryable, Selectable, Serialize, Clone)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub tel: String,
    pub rib: String,
    pub email_paypal: String,
    pub tel_wero: String,
    pub profil_pict_ref: String,
    pub password: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

// Struct pour les INSERTS sans les champs auto-générés
#[derive(Insertable, Deserialize, Clone)]
#[diesel(table_name = crate::schema::users)]
pub struct InsertableUser {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub tel: String,
    pub rib: String,
    pub email_paypal: String,
    pub tel_wero: String,
    pub profil_pict_ref: String,
    pub password: String,
}