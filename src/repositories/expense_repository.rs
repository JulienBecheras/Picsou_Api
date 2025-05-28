use diesel::prelude::*;
use rocket::http::Status;
use projet_picsou_api::establish_connection;
use crate::models::contributor::ContributorUserWithStatus;
use crate::models::expense::{DetailExpenseFlat, Expense};
use crate::models::group::InsertableGroup;
use crate::models::participant::ParticipantUserWithStatus;
use crate::models::refund::Refund;
use diesel::sql_types::{Int4};
/*pub fn insert_expense_repository(insertable_expense: InsertableExpense) -> Result<Expense, (Status, String)> {
    let conn = &mut establish_connection();

    match diesel::insert_into(expenses).values(&insertable_expense).get_result::<Expense>(conn) {
        Ok(expense) => Ok(expense),
        Err(_) => Err((Status::InternalServerError, "An internal server error occurred while querying the database".to_string())),
    }
}*/

pub fn get_expenses_by_group_id(group_id: &i32) -> Result<Vec<DetailExpenseFlat>, (Status, String)> {
    let conn = &mut establish_connection();

    match diesel::sql_query("
    SELECT  g.id AS group_id, g.name AS group_name, g.pict_ref AS pict_ref, g.created_at AS created_at,
            e.id AS expense_id, e.name AS expense_name, e.description AS expense_description, e.montant AS montant, e.stock_parts AS stock_parts, e.created_at AS expense_created_at, e.updated_at AS expense_updated_at,
            c.id AS contributor_id, c.amount_contributed AS amount_contributed, c.groups_users_id AS contributor_group_user_id,
            p.id AS participant_id, p.amount_participated AS amount_participated, p.groups_users_id AS participant_group_user_id, p.part_number AS part_number,
        r.id AS refund_id, r.amount AS refund_amount, r.status AS refund_status, r.contributors_id AS refund_contributor_id, r.participants_id AS refund_participant_id, r.created_at AS refund_created_at, r.updated_at AS refund_updated_at
    FROM groups_users ug
    JOIN groups g ON ug.id_group = g.id
    JOIN contributors c ON c.groups_users_id = ug.id
    JOIN expenses e ON c.expenses_id = e.id
    LEFT JOIN participants p ON p.expenses_id = e.id
    LEFT JOIN refunds r ON r.participants_id = p.id
    WHERE g.id = $1
    GROUP BY g.id, e.id, c.id, p.id, r.id
    ORDER BY g.id, e.id, c.id, p.id, r.id
    ")
    .bind::<Int4, _>(*group_id)
    .get_results::<DetailExpenseFlat>(conn){
        Ok(expenses_result) => Ok(expenses_result),
        Err(_) => Err((Status::BadRequest, "No expenses found.".to_string())),
    }
}

pub struct DetailExpense {
    pub group: InsertableGroup,
    pub contributors: Vec<ContributorUserWithStatus>,
    pub participants: Vec<ParticipantUserWithStatus>,
    pub expense: Expense,
    pub refunds: Vec<Refund>
}