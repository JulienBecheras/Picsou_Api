use crate::models::expense::{Expense, DetailExpense, DetailExpenseFlat};
use core::result::Result::Err;
use std::vec;
use rocket::http::Status;
use rocket::yansi::Paint;
use crate::auth::AuthenticatedUser;
use crate::models::contributor::{Contributor, ContributorUserWithStatus};
use crate::models::group::InsertableGroup;
use crate::models::participant::{Participant, ParticipantUserWithStatus};
use crate::models::refund::Refund;
use crate::models::user::UserWithStatus;
use crate::repositories::expense_repository;
use crate::services::group_service::{is_user_member_of_group, get_users_group_service};

/*pub fn create_expense_service(mut insertable_expense: InsertableExpense) -> Result<Expense, (rocket::http::Status, String)> {
    let mut conn = establish_connection();

    conn.transaction(|conn| {
        // On extrait les participants/contributeurs pour ne pas les passer à Diesel
        let participants = std::mem::take(&mut insertable_expense.participants);
        let contributors = std::mem::take(&mut insertable_expense.contributors);

        // Insertion de la dépense seule
        let expense = insert_expense_repository(conn, insertable_expense)?;

        // Insertion des participants
        for mut participant in participants {
            participant.expense_id = expense.id;
            insert_participant(conn, participant)?;
        }

        // Insertion des contributeurs
        for mut contributor in contributors {
            contributor.expense_id = expense.id;
            insert_contributor(conn, contributor)?;
        }

        Ok(expense)
    }).map_err(|e: Error| (rocket::http::Status::InternalServerError, e.to_string()))
}*/

pub fn get_all_expenses_group(group_id: &i32, authenticated_user: &AuthenticatedUser) -> Result<Vec<DetailExpense>, (Status, String)> {
    if is_user_member_of_group(group_id, authenticated_user.user_id){
        match expense_repository::get_expenses_by_group_id(group_id) {
            Ok(expenses) => {
                match normalize_detail_expenses_flat(&expenses, authenticated_user) {
                    Ok(detail_expenses) => Ok(detail_expenses),
                    Err(e) => Err(e)
                }
            }
            Err(e) => Err(e)
        }
    }else { return Err((Status::InternalServerError, "No expense found".to_owned())) }
}

pub fn normalize_detail_expenses_flat(list_detail_expenses_flat: &Vec<DetailExpenseFlat>, authenticated_user: &AuthenticatedUser) -> Result<Vec<DetailExpense>, (Status, String)> {
    if list_detail_expenses_flat.is_empty() {
        return Ok(vec![]);
    }

    let first_expense = list_detail_expenses_flat.first().unwrap();
    let group = InsertableGroup {
        id: Some(first_expense.group_id),
        name: first_expense.group_name.clone(),
        pict_ref: first_expense.pict_ref.clone(),
        created_at: first_expense.created_at,
    };

    let group_id = group.id.ok_or_else(|| {
        (Status::BadRequest, "Group ID is missing in detail_expense_flat".to_string())
    })?;

    let users = get_users_group_service(&group_id, authenticated_user)?;

    let mut detail_expenses: Vec<DetailExpense> = vec![];
    let mut current_expense_id: Option<i32> = None;

    let mut contributors: Vec<Contributor> = vec![];
    let mut participants: Vec<Participant> = vec![];
    let mut refunds: Vec<Refund> = vec![];
    let mut current_expense: Option<Expense> = None;

    for item in list_detail_expenses_flat {
        if let Some(expense_id) = item.expense_id {
            if current_expense_id != Some(expense_id) {
                // Sauvegarde l'ancienne dépense si elle existe
                if let Some(expense) = current_expense.take() {
                    detail_expenses.push(DetailExpense {
                        group: group.clone(),
                        contributors: contributor_to_contributor_user_status(&users, &contributors),
                        participants: participant_to_participant_user_status(&users, &participants),
                        expense,
                        refunds: refunds.clone(),
                    });

                    contributors = Vec::new();
                    participants = Vec::new();
                    refunds = Vec::new();
                }

                // Crée une nouvelle expense
                current_expense = Some(Expense {
                    id: expense_id,
                    name: item.expense_name.clone().unwrap_or_default(),
                    description: item.expense_description.clone(),
                    montant: item.montant.expect("Montant is missing for expense"),
                    stock_parts: item.stock_parts.expect("stock part is missing for expense"),
                    created_at: item.expense_created_at.expect("created at is missing for expense"),
                    updated_at: item.expense_updated_at.expect("updated at is missing for expense"),
                });

                current_expense_id = Some(expense_id);
            }
        }

        if let Some(contributor_id) = item.contributor_id {
            if contributors.iter().all(|c| c.id != contributor_id) {
                contributors.push(Contributor {
                    id: contributor_id,
                    amount_contributed: item.amount_contributed.expect("amount_contributed missing"),
                    groups_users_id: item.contributor_group_user_id.expect("contributor_group_user missing"),
                    expenses_id: item.expense_id.expect("expense_id missing"),
                });
            }
        }

        if let Some(participant_id) = item.participant_id {
            if participants.iter().all(|p| p.id != participant_id) {
                participants.push(Participant {
                    id: participant_id,
                    amount_participated: item.amount_participated.expect("amount_participated missing"),
                    groups_users_id: item.participant_group_user_id.expect("participant_group_user missing"),
                    part_number: item.part_number,
                    expenses_id: item.expense_id.expect("expense_id is missing"),
                });
            }
        }

        if let Some(refund_id) = item.refund_id {
            if refunds.iter().all(|r| r.id != refund_id) {
                refunds.push(Refund {
                    id: refund_id,
                    amount: item.refund_amount.expect("amount_refund is missing"),
                    status: item.refund_status.clone().expect("refund_status is missing"),
                    contributors_id: item.refund_contributor_id.expect("refund_contributor_id missing"),
                    participants_id: item.refund_participant_id.expect("refund_participant_id missing"),
                    created_at: item.refund_created_at,
                    updated_at: item.refund_updated_at,
                });
            }
        }
    }

    if let Some(expense) = current_expense {
        detail_expenses.push(DetailExpense {
            group,
            contributors: contributor_to_contributor_user_status(&users, &contributors),
            participants: participant_to_participant_user_status(&users, &participants),
            expense,
            refunds,
        });
    }

    Ok(detail_expenses)
}


pub fn contributor_to_contributor_user_status(users: &Vec<UserWithStatus>, contributors: &Vec<Contributor>) -> Vec<ContributorUserWithStatus>{
    let mut contributor_with_status: Vec<ContributorUserWithStatus> = Vec::new();

    for contributor in contributors {
        for user in users {
            if contributor.groups_users_id == user.group_user_id {
                contributor_with_status.push(ContributorUserWithStatus{user: user.user.clone(), contributor: *contributor, status: user.status });
            }
        }
    }
    contributor_with_status
}

pub fn participant_to_participant_user_status(users: &Vec<UserWithStatus>, participants: &Vec<Participant>) -> Vec<ParticipantUserWithStatus> {
    let mut participant_with_status: Vec<ParticipantUserWithStatus> = Vec::new();

    for participant in participants {
        for user in users {
            if participant.groups_users_id == user.group_user_id {
                participant_with_status.push(ParticipantUserWithStatus{user: user.user.clone(), participant: participant.clone(), status: user.status});
            }
        }
    }
    participant_with_status
}