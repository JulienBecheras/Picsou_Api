use diesel::prelude::*;
use rocket::serde::Deserialize;
use serde::Serialize;
use crate::models::user::PublicUser;

#[derive(Queryable, Selectable, Serialize, Clone, AsChangeset, Deserialize, Copy)]
#[diesel(table_name = crate::schema::contributors)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Contributor {
    pub id: i32,
    pub amount_contributed: f64,
    pub groups_users_id: i32,
    pub expenses_id: i32,
}

// Struct pour les INSERTS sans les champs auto-générés
#[derive(Insertable, Deserialize, Clone, Serialize)]
#[diesel(table_name = crate::schema::contributors)]
pub struct InsertableContributor {
    pub amount_contributed: f64,
    pub groups_users_id: i32,
    pub expenses_id: Option<i32>,
}

#[derive(Deserialize, Clone, Serialize)]
pub struct ContributorUserWithStatus {
    pub user: PublicUser,
    pub status: i32,

    pub contributor: Contributor,
}
