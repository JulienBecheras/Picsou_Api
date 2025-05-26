use rocket::http::Status;
use projet_picsou_api::establish_connection;
use crate::models::group::{Group, InsertableGroup};
use crate::schema::groups::dsl::{groups, id};
use diesel::prelude::*;

pub fn get_group_by_id(groups_id: &i32) -> Result<Group, (Status, String)> {
    let conn = &mut establish_connection();

    match groups.filter(id.eq(groups_id)).first::<Group>(conn) {
        Ok(group) => Ok(group), //If group is found, we return the first element
        Err(diesel::result::Error::NotFound) => Err((Status::NotFound, "This group does not exist".to_string())),
        Err(_) => Err((Status::InternalServerError, "An internal server error occurred while querying the database".to_string())),
    }
}

pub fn insert_group_transac(conn: &mut PgConnection, group: &InsertableGroup) -> Result<Group, (Status, String)> {
    match diesel::insert_into(groups).values(group).get_result::<Group>(conn) {
        Ok(group) => Ok(group),
        Err(_) => Err((Status::InternalServerError, "An internal server error occurred while querying the database".to_string())),
    }
}

pub fn insert_group(group: &InsertableGroup) -> Result<Group, (Status, String)> {
    let conn = &mut establish_connection();
    insert_group_transac(conn, group)
}