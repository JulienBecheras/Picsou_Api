use rocket::http::Status;
use projet_picsou_api::establish_connection;
use crate::models::group::Group;
use crate::models::user::User;
use crate::schema::users::dsl::users;
use crate::schema::groups::dsl::groups;

pub fn get_group_by_id(id: &i32) -> Result<Group, (Status, String)> {
    let conn = &mut establish_connection();

    match groups.filter(crate::schema::groups::id.eq(&id))
        .first::<Group>(conn) {
        Ok(group) => Ok(group), //If group is found, we return the first element
        Err(diesel::result::Error::NotFound) => Err((Status::NotFound, format!("Group with id {} does not exist", id))),
        Err(_) => Err((Status::InternalServerError, "An internal server error occurred while querying the database".to_string())),
    }
}