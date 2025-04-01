use diesel::prelude::*;

#[derive(Queryable, Selectable)]
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
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::friends)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Friend {
    pub id: i32,
    pub user1_id: i32,
    pub user2_id: i32,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}