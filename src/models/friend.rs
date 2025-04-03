use diesel::prelude::*;
use chrono::NaiveDateTime;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::friends)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Friend {
    pub id: i32,
    pub user1_id: i32,
    pub user2_id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}