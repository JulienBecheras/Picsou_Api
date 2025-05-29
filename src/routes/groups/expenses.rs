use rocket::http::Status;
use rocket::serde::json::Json;
use crate::auth::AuthenticatedUser;
use crate::services::expense_service;
use crate::models::expense::{Expense, DetailExpense};

/**
Recupérère toutes les dépenses d'un utilisateur.
*/
#[get ("/expenses", format = "application/json")]
pub fn get_all_exepenses(authenticated_user: AuthenticatedUser) -> Result<Json<Vec<DetailExpense>>, (Status, String)> {
    match expense_service::get_all_expenses_service(&authenticated_user) {
        Ok(expenses) => Ok(Json(expenses)),
        Err((status, message)) => Err((status, message)),
    }
}

#[get ("/<group_id>/expenses", format = "application/json")]
pub fn get_all_expenses_in_group(group_id: i32, authenticated_user: AuthenticatedUser) -> Result<Json<Vec<DetailExpense>>, (Status, String)> {
    match expense_service::get_all_expenses_group(&group_id, &authenticated_user) {
        Ok(users) => Ok(Json(users)),
        Err((status, message)) => Err((status, message)),
    }
}

/*
Récupère une dépense par son ID.
 */
#[get ("/<group_id>/expenses/<expense_id>", format = "application/json")]
pub fn get_expense_by_id(group_id: i32, expense_id: i32, authenticated_user: AuthenticatedUser) -> Result<Json<DetailExpense>, (Status, String)> {
    match expense_service::get_expense_by_id(&group_id, &expense_id, &authenticated_user) {
        Ok(expense) => Ok(Json(expense)),
        Err((status, message)) => Err((status, message)),
    }
}