use rocket::http::Status;
use crate::auth::AuthenticatedUser;
use crate::services::group_service;
use crate::models::refund::BalanceGroup;
use crate::repositories::expense_repository;

pub fn get_balance_in_group_service(group_id: i32, authenticated_user: &AuthenticatedUser) -> Result<BalanceGroup, (Status, String)> {
    let user_status = match group_service::is_user_member_of_group_get_status(&group_id, authenticated_user.user_id){
        Ok(status) => status,
        Err(status) => return Err(status),
    };
    let cost_group = match get_total_cost_group(group_id) { 
        Ok(total_cost) => total_cost,
        Err(status) => return Err(status),
    };
    if user_status == 5 {
        // mettre balance, total cost, total contributed user à 0 et retourner les couts du groupe 
        return Ok(BalanceGroup{
            group_id,
            total_cost_group: cost_group,
            total_cost_user: 0.0,
            total_contributed_user: 0.0,
            balance_user: 0.0,
        });
    }
    
    let total_contributed_user = match expense_repository::get_total_contributed_user(group_id, authenticated_user.user_id) {
        Ok(total_contributed) => total_contributed,
        Err(status) => return Err(status),
    };
    
    let total_cost_user = match expense_repository::get_total_cost_user(group_id, authenticated_user.user_id) {
        Ok(total_cost) => total_cost,
        Err(status) => return Err(status),
    };
    
    let total_refund_amount = match get_total_refund_amount(group_id, authenticated_user.user_id) {
        Ok(total_refund) => total_refund,
        Err(status) => return Err(status),
    };
    
    Ok(BalanceGroup{
        group_id,
        total_cost_group: cost_group.round()*100.0 / 100.0,
        total_cost_user: total_cost_user.round()*100.0 / 100.0,
        total_contributed_user: total_contributed_user.round()*100.0 / 100.0,
        balance_user: (total_contributed_user - total_cost_user).round()*100.0 / 100.0
    })
}

pub fn get_total_cost_group(group_id: i32) -> Result<f64, (Status, String)> {
    match expense_repository::get_total_cost_group(group_id) {
        Ok(total_cost) => Ok(total_cost),
        Err(status) => Err(status),
    }
}

pub fn get_total_refund_amount(group_id: i32, user_id: i32) -> Result<f64, (Status, String)> {
    match expense_repository::get_total_refund_amount(group_id, user_id) {
        Ok(total_refund) => Ok(total_refund),
        Err(status) => Err(status),
    }
}

/*pub fn get_all_refunds_user_is_participant_contributors(group_id: i32, user_id: i32) -> Result<Vec<Refund>, (Status, String)> {
    match expense_repository::get_all_refund_between_user(user_id, user_id) {
        Ok(refunds) => Ok(refunds),
        Err(status) => Err(status),
    }
}*/

