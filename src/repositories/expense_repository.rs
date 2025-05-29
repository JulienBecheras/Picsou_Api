use diesel::prelude::*;
use rocket::http::Status;
use projet_picsou_api::establish_connection;
use crate::models::expense::{DetailExpenseFlat, Expense, InsertableExpense};
use diesel::sql_types::{Int4};
use crate::models::refund::{TotalContributed, TotalCost, TotalParticipated, TotalRefundAmount};
use crate::schema::expenses::dsl::{expenses};
pub fn insert_expense_repository(insertable_expense: InsertableExpense, conn: &mut diesel::PgConnection) -> Result<Expense, (Status, String)> {
    match diesel::insert_into(expenses).values(&insertable_expense).get_result::<Expense>(conn) {
        Ok(expense) => Ok(expense),
        Err(_) => Err((Status::InternalServerError, "An internal server error occurred while querying the database".to_string())),
    }
}

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


pub fn get_expenses_by_id(group_id: &i32) -> Result<Vec<DetailExpenseFlat>, (Status, String)> {
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
    WHERE e.id = $1
    GROUP BY g.id, e.id, c.id, p.id, r.id
    ORDER BY g.id, e.id, c.id, p.id, r.id
    ")
        .bind::<Int4, _>(*group_id)
        .get_results::<DetailExpenseFlat>(conn){
        Ok(expenses_result) => Ok(expenses_result),
        Err(_) => Err((Status::BadRequest, "No expenses found.".to_string())),
    }
}

pub fn get_total_cost_group(group_id: i32) -> Result<f64, (Status, String)> {
    let conn = &mut establish_connection();

    match diesel::sql_query("
        SELECT ROUND(SUM(e.montant)::numeric, 2)::double precision AS total_cost
        FROM expenses e
        JOIN contributors c ON e.id = c.expenses_id
        JOIN groups_users gu ON c.groups_users_id = gu.id
        WHERE gu.id_group = $1
    ")
    .bind::<Int4, _>(group_id)
    .get_result::<TotalCost>(conn) {
        Ok(total_cost) => Ok(total_cost.total_cost.unwrap_or(0.0)),
        Err(e) => Err((Status::InternalServerError, format!("Erreur Diesel: {:?}", e))),
    }
}

pub fn get_total_contributed_user(group_id: i32, user_id: i32) -> Result<f64, (Status, String)> {
    let conn = &mut establish_connection();

    match diesel::sql_query("
        SELECT ROUND(SUM(c.amount_contributed)::numeric, 2)::double precision AS total_contributed
        FROM contributors c
        JOIN groups_users gu ON c.groups_users_id = gu.id
        JOIN users u ON gu.id_user = u.id
        WHERE gu.id_group = $1 AND u.id = $2
    ")
    .bind::<Int4, _>(group_id)
    .bind::<Int4, _>(user_id)
    .get_result::<TotalContributed>(conn) {
        Ok(total_contributed) => Ok(total_contributed.total_contributed.unwrap_or(0.0)),
        Err(e) => Err((Status::InternalServerError, format!("Erreur Diesel: {:?}", e))),    }
}

pub fn get_total_cost_user(group_id: i32, user_id: i32) -> Result<f64, (Status, String)> {
    let conn = &mut establish_connection();

    match diesel::sql_query("
        SELECT ROUND(SUM(p.amount_participated)::numeric, 2)::double precision AS total_participated
        FROM participants p
        JOIN groups_users gu ON p.groups_users_id = gu.id
        JOIN users u ON gu.id_user = u.id
        WHERE gu.id_group = $1 AND u.id = $2
    ")
    .bind::<Int4, _>(group_id)
    .bind::<Int4, _>(user_id)
    .get_result::<TotalParticipated>(conn) {
        Ok(total_cost) => Ok(total_cost.total_participated.unwrap_or(0.0)),
        Err(_) => Err((Status::InternalServerError, "An internal server error occurred while querying the database".to_string())),
    }
}

pub fn get_total_refund_amount(group_id: i32, user_id: i32) -> Result<f64, (Status, String)> {
    let conn = &mut establish_connection();

    match diesel::sql_query("
        SELECT ROUND(SUM(r.amount)::numeric, 2)::double precision AS total_refund
        FROM refunds r
        JOIN participants p ON r.participants_id = p.id
        JOIN groups_users gu ON p.groups_users_id = gu.id
        JOIN users u ON gu.id_user = u.id
        WHERE gu.id_group = $1 AND u.id = $2
    ")
    .bind::<Int4, _>(group_id)
    .bind::<Int4, _>(user_id)
    .get_result::<TotalRefundAmount>(conn) {
        Ok(total_refund) => Ok(total_refund.total_refund.unwrap_or(0.0)),
        Err(_) => Err((Status::InternalServerError, "An internal server error occurred while querying the database".to_string())),
    }
}

// pub fn get_all_refund_between_user()