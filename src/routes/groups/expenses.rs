use rocket::http::Status;
use rocket::serde::json::Json;
use crate::auth::AuthenticatedUser;
use crate::services::expense_service;
use crate::models::expense::{Expense, DetailExpense, InsertableExpense, InsertableDetailExpense};

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

/**
Crée une dépense.

Vérifie le niveau de privilège de l'utilisateur dans le groupe avant de créer la dépense.
Verifie que tous les participants et contributeurs de la dépense sont bien dans le groupe et ne sont pas spectateur.
Verifie qu'il n'y a pas de doublon de contributeurs.
Verifie si un contributeur est aussi partcipant à la dépense, un remboursement est créé pour lui.
Verifie que les montants sont positifs et que le montant total des participants est égal au montant total des contributeurs et au montant de la dépense sauf si il y a des parts dans la dépense.

*/
#[post ("/<group_id>/expenses", format = "application/json", data = "<expense>")]
pub fn create_expense_to_group(group_id: i32, expense: Json<InsertableDetailExpense>, authenticated_user: AuthenticatedUser) -> Result<Json<DetailExpense>, (Status, String)> {
    match expense_service::create_expense_to_group_service(&group_id, &expense.into_inner(), authenticated_user) {
        Ok(expense) => Ok(Json(expense)),
        Err((status, message)) => Err((status, message)),
    }
}