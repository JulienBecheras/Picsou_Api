use diesel::RunQueryDsl;
use rocket::http::Status;
use crate::models::refund::{InsertableRefund, Refund};
use crate::schema::refunds;

pub fn insert_all_refunds(refunds_values: &mut Vec<InsertableRefund>, conn: &mut diesel::PgConnection) -> Result<Vec<Refund>, (Status, String)> {
    let mut list_of_refunds: Vec<Refund> = vec![];
    
    for refund in refunds_values.iter() {
        match diesel::insert_into(refunds::table).values(refund).get_result::<Refund>(conn) {
            Ok(refund) => list_of_refunds.push(refund),
            Err(diesel::result::Error::DatabaseError(diesel::result::DatabaseErrorKind::UniqueViolation, _)) => {
                return Err((Status::Conflict, "One or more refunds already exist".to_string()))
            },
            Err(_) => return Err((Status::InternalServerError, "An internal server error occurred while inserting refunds".to_string())),
        }
    }
    Ok(list_of_refunds)
}