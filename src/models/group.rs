use chrono::NaiveDateTime;
use diesel::{AsChangeset, Insertable, Queryable, Selectable};
use rocket::serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Serialize, Clone, AsChangeset, Deserialize)]
#[diesel(table_name = crate::schema::groups)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Group {
    pub id: i32,
    pub name: String,
    pub pict_ref: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

// Struct pour les INSERTS sans les champs auto-générés
#[derive(Insertable, Deserialize, Clone, Serialize)]
#[diesel(table_name = crate::schema::groups)]
pub struct InsertableGroup {
    pub id: Option<i32>,
    pub name: String,
    pub pict_ref: String,
    pub created_at: NaiveDateTime,
}



// Struct pour les INSERTS sans les champs auto-générés
#[derive(Deserialize, Clone)]
pub struct UserIdWithStatus {
    pub id_user: i32,
    pub status: i32,
}


#[derive(Deserialize, Clone)]
pub struct GroupWithUser {
    pub group: InsertableGroup,
    pub users: Vec<UserIdWithStatus>,
}

#[derive(Insertable, Deserialize, Clone, Serialize, AsChangeset)]
#[diesel(table_name = crate::schema::groups)]
pub struct UpdatableGroup {
    pub name: Option<String>,
    pub pict_ref: Option<String>,
}