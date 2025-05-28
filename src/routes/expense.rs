
/*#[post("/", format = "application/json", data = "<insertable_expense>")]
pub fn create_expense(insertable_expense: Json<InsertableExpense>, _auth: AuthenticatedUser) -> Result<Json<Expense>, (Status, String)> {
    match create_expense_service(insertable_expense.into_inner()) {
        Ok(expense) => Ok(Json(expense)),
        Err(e) => Err((Status::InternalServerError, e.to_string())),
    }
}

#[put("/", format = "application/json", data = "<expense>")]
pub fn update_expense(expense: Json<Expense>, _auth: AuthenticatedUser) -> Result<Json<Expense>, (Status, String)> {
}
 */