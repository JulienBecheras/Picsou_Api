use diesel::RunQueryDsl;
use rocket::http::Status;
use crate::models::participant::{InsertableParticipant, Participant};
use crate::schema::participants::dsl::participants;
pub fn insert_all_participants(participants_values: Vec<InsertableParticipant>, conn: &mut diesel::PgConnection) -> Result<Vec<Participant>, (Status, String)> {
    let mut list_of_participants: Vec<Participant> = vec![];
    for participant in participants_values {
        match diesel::insert_into(participants).values(&participant).get_result::<Participant>(conn) {
            Ok(participant) => list_of_participants.push(participant),
            Err(diesel::result::Error::DatabaseError(diesel::result::DatabaseErrorKind::UniqueViolation, _)) => return Err((Status::Conflict, "One or more participants already exist".to_string())),
            Err(_) => return Err((Status::InternalServerError, "An internal server error occurred while inserting participants".to_string())),
        }
    }
    Ok(list_of_participants)
}