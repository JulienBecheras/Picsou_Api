use chrono::NaiveDateTime;
use diesel::{AsChangeset, Insertable, Queryable, Selectable};
use rocket::serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Serialize, Clone, AsChangeset, Deserialize)]
#[diesel(table_name = crate::schema::groups_users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct GroupUser {
    pub id: i32,
    pub id_user: i32,
    pub id_group: i32,
    pub status: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

// Struct pour les INSERTS sans les champs auto-générés
#[derive(Insertable, Deserialize, Clone)]
#[diesel(table_name = crate::schema::groups)]
pub struct InsertableGroupUser {
    pub id_user: i32,
    pub id_group: i32,
    pub status: i32,
}