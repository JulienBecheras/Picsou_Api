use chrono::NaiveDateTime;
use diesel::{AsChangeset, Insertable, Queryable, Selectable};
use rocket::serde::{Deserialize, Serialize};
use crate::models::user::PublicUser;

#[derive(Queryable, Selectable, Serialize, Clone, AsChangeset, Deserialize)]
#[diesel(table_name = crate::schema::participants)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Participant {
    pub id: i32,
    pub amount_participated: f64,
    pub groups_users_id: i32,
    pub part_number: Option<i32>,
    pub expenses_id: i32,
}

#[derive(Insertable, Deserialize, Clone)]
#[diesel(table_name = crate::schema::participants)]
pub struct InsertableParticipant {
    pub amount_participated: f64,
    pub groups_users_id: i32,
    pub part_number: Option<i32>,
    pub expenses_id: i32,
}

#[derive(Deserialize, Clone, Serialize)]
pub struct ParticipantUserWithStatus {
    pub user: PublicUser,
    pub status: i32,
    pub participant: Participant,
}