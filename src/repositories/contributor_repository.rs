use diesel::RunQueryDsl;
use rocket::http::Status;
use crate::models::contributor::{Contributor, InsertableContributor};
use crate::schema::contributors::dsl::contributors;

pub fn insert_all_contributors(contributors_values: Vec<InsertableContributor>, conn: &mut diesel::PgConnection) -> Result<(Vec<Contributor>), (Status, String)> {
    let mut list_of_contributors: Vec<Contributor> = vec![];
    for contributor in contributors_values {
        match diesel::insert_into(contributors).values(contributor).get_result::<Contributor>(conn){
            Ok(contributor) => list_of_contributors.push(contributor),
            Err(diesel::result::Error::DatabaseError(
                    diesel::result::DatabaseErrorKind::UniqueViolation, _)) => return Err((Status::Conflict, "One or more contributors already exist".to_string())),
            Err(_) => return Err((Status::InternalServerError, "An internal server error occurred while inserting contributors".to_string())),
        }
    }
    Ok(list_of_contributors)
}