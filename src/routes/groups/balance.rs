use rocket::http::Status;
use rocket::serde::json::Json;
use crate::auth::AuthenticatedUser;
use crate::models::refund::BalanceGroup;
use crate::services::refund_service;

#[get("/<group_id>/balance", format = "application/json")]
pub fn get_balance_in_group(group_id: i32, authenticated_user: AuthenticatedUser) -> Result<Json<BalanceGroup>, (Status, String)>{
    match refund_service::get_balance_in_group_service(group_id, &authenticated_user){
        Ok(balance_in_group) => Ok(Json(balance_in_group)),
        Err(status) => Err(status),
    }
}