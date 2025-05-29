use crate::models::expense::{Expense, DetailExpense, DetailExpenseFlat, InsertableDetailExpense, InsertableExpense};
use core::result::Result::Err;
use std::vec;
use diesel::Connection;
use rocket::http::Status;
use rocket::yansi::Paint;
use projet_picsou_api::establish_connection;
use crate::auth::AuthenticatedUser;
use crate::models::contributor::{Contributor, ContributorUserWithStatus, InsertableContributor};
use crate::models::group::InsertableGroup;
use crate::models::participant::{InsertableParticipant, Participant, ParticipantUserWithStatus};
use crate::models::refund::{InsertableRefund, Refund};
use crate::models::user::UserWithStatus;
use crate::repositories::expense_repository;
use crate::repositories::group_user_repository::get_users_group;
use crate::services::group_service::{is_user_member_of_group, get_users_group_service, get_all_groups_service, is_user_member_of_group_get_status};
use crate::repositories::contributor_repository;
use crate::repositories::participant_repository;
use crate::repositories::refund_repository;

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
        created_at: Some(first_expense.created_at),
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

pub fn get_all_expenses_service(authenticated_user: &AuthenticatedUser) -> Result<Vec<DetailExpense>, (Status, String)> {
    let groups = match get_all_groups_service(&authenticated_user){
        Ok(groups) => groups,
        Err(e) => return Err(e),
    };

    let mut all_expenses: Vec<DetailExpense> = vec![];

    for group in groups {
        all_expenses.append(&mut match get_all_expenses_group(&group.id, authenticated_user){
            Ok(expenses) => {
                expenses
            },
            Err(e) => {
                return Err(e);
            }
        });
    }

    Ok(all_expenses)
}

pub fn get_expense_by_id(group_id: &i32, expense_id: &i32, authenticated_user: &AuthenticatedUser) -> Result<DetailExpense, (Status, String)> {
    let expenses = match expense_repository::get_expenses_by_id(expense_id) {
        Ok(expenses) => expenses,
        Err(e) => return Err(e),
    };
    
    if expenses.first().unwrap().group_id != *group_id{
        return Err((Status::BadRequest, "Expense does not belong to the specified group".to_string()));
    }
    
    if is_user_member_of_group(&expenses.first().unwrap().group_id, authenticated_user.user_id) { 
        match normalize_detail_expenses_flat(&expenses, authenticated_user) {
            Ok(detail_expenses) => {
                if let Some(expense) = detail_expenses.first() {
                    Ok(expense.clone())
                } else {
                    Err((Status::NotFound, "Expense not found".to_owned()))
                }
            },
            Err(e) => Err(e),
        }
    }else { Err((Status::Unauthorized, "User is not a member of the group".to_string())) }
}

/**
Vérifie le niveau de privilège de l'utilisateur dans le groupe avant de créer la dépense.
Verifie que tous les participants et contributeurs de la dépense sont bien dans le groupe et ne sont pas spectateur.
Verifie qu'il n'y a pas de doublon de contributeurs.
Verifie si un contributeur est aussi partcipant à la dépense, un remboursement est créé pour lui.
Verifie que les montants sont positifs et que le montant total des participants est égal au montant total des contributeurs et au montant de la dépense sauf si il y a des parts dans la dépense.

*/
pub fn create_expense_to_group_service(group_id: &i32, insertable_detail_expense: &InsertableDetailExpense, authenticated_user: AuthenticatedUser) -> Result<DetailExpense, (Status, String)> {
    
    // Recupération des user groups
    let users_groups = match get_users_group(group_id) {
        Ok(users_groups) => users_groups,
        Err(e) => return Err(e),
    };
    
    // Verification du niveau de privilège de l'utilisateur dans le groupe
    let status_user = match is_user_member_of_group_get_status(group_id, authenticated_user.user_id) {
        Ok(status) => status,
        Err(e) => return Err(e),
    };
    
    let group_id_user = match users_groups.iter().find(|&g| g.id_user == authenticated_user.user_id) {
        Some(group_user) => group_user.id_group,
        None => return Err((Status::Unauthorized, "User is not a member of the group".to_string())),
    };
    
    // Verification que l'utilisateur possède les droits pour créer une dépense dans le groupe
    let mut is_participating = true;
    if status_user > 4 { 
        return Err((Status::Unauthorized, "User is just a spectator of the group".to_string()));
    }else if status_user == 4 || status_user == 3 { 
        is_participating = false;
    }

    // Verification qu'il n'y a pas de doublon de contributeurs
    if is_contributor_contain_duplicate(&insertable_detail_expense.contributors) {
        return Err((Status::BadRequest, "Contributor contains duplicate".to_string()));
    }
    
    // Verification du niveau de privilège des contributeurs dans le groupe
    let mut somme_contributed: f64 = 0.0;
    for insertable_contributor in &insertable_detail_expense.contributors {
        let mut i = 0;
        let mut finded = false;
        while (i < users_groups.len() && !finded) {
            if users_groups[i].id == insertable_contributor.groups_users_id {
                if users_groups[i].id == group_id_user {   // Verification que l'utilisateur participe au groupe
                    is_participating = true;
                }
                if users_groups[i].status >= 0 && users_groups[i].status <= 4 {
                    if insertable_contributor.amount_contributed < 0.0 {
                        return Err((Status::BadRequest, "Contributor amount must be positive".to_string()));
                    }
                    somme_contributed += insertable_contributor.amount_contributed;
                    finded = true;
                }else{
                    return Err((Status::BadRequest, "Contributor is a spectator".to_string()));
                }
            }
            i += 1;
        }
        if !finded {
            return Err((Status::BadRequest, "Contributor is not a member of the group".to_string()));
        }
    }

    // Verification du niveau de privilège des participants dans le groupe
    let mut somme_participated: f64 = 0.0;
    let mut somme_part_number: i32 = 0;
    let mut refunds_list: Vec<InsertableRefund> = vec![];
    for insertable_participant in &insertable_detail_expense.participants {
        let mut i = 0;
        let mut finded = false;
        while (i < users_groups.len() && !finded) {
            if users_groups[i].id == insertable_participant.groups_users_id {
                if users_groups[i].id == group_id_user {   // Verification que l'utilisateur participe au groupe
                    is_participating = true;
                }
                if users_groups[i].status >= 0 && users_groups[i].status <= 4 {
                    // Si le participant est aussi contributeur, on crée un remboursement pour lui
                    match participant_is_contributor(insertable_participant, &insertable_detail_expense.contributors){
                        Ok(montant) => {
                            refunds_list.push(InsertableRefund {
                                amount: calculate_amount(&montant, &insertable_participant.amount_participated),
                                contributors_id: insertable_participant.groups_users_id,
                                participants_id: insertable_participant.groups_users_id,
                                status: "completed".to_string(),
                                created_at: None,
                            });
                        },
                        Err(_) => {
                            // Si le participant n'est pas contributeur, on ne fait rien
                        }
                    }
                    if insertable_participant.amount_participated < 0.0 {
                        return Err((Status::BadRequest, "Participant amount must be positive".to_string()));
                    }
                    if let Some(part_number) = insertable_participant.part_number {
                        if part_number < 0 {
                            return Err((Status::BadRequest, "Participant part number must be positive".to_string()));
                        }
                        somme_part_number += part_number;
                    }
                    somme_participated += insertable_participant.amount_participated;
                    finded = true;
                }else{
                    return Err((Status::BadRequest, "Participant is a spectator".to_string()));
                }
            }
            i += 1;
        }
        if !finded {
            return Err((Status::BadRequest, "Participant is not a member of the group".to_string()));
        }
    }
    
    if !is_participating {
        return Err((Status::Unauthorized, "User is not participating in the group and don't get inof ".to_string()));
    }
    
    // Verification que le montant total des participants est égal au montant total des contributeurs et au montant de la dépense sauf si il y a des parts dans la dépense
    if insertable_detail_expense.expense.montant <= 0.0 { 
        return Err((Status::BadRequest, "Expense amount must be positive".to_string()));
    }else if insertable_detail_expense.expense.stock_parts < 0 { 
        return Err((Status::BadRequest, "Expense stock parts must be positive".to_string()));
    }else if insertable_detail_expense.expense.montant != somme_contributed{
        return Err((Status::BadRequest, "Total amount contributed does not match the expense amount".to_string()));
    }else if (insertable_detail_expense.expense.montant != somme_participated && insertable_detail_expense.expense.stock_parts == 0) || (insertable_detail_expense.expense.montant < somme_participated && insertable_detail_expense.expense.stock_parts != 0) { 
        return Err((Status::BadRequest, "Total amount participated does not match the expense amount".to_string()));
    }else if insertable_detail_expense.expense.stock_parts > 0 && somme_part_number > insertable_detail_expense.expense.stock_parts {
        return Err((Status::BadRequest, "Total part number does not match the expense stock parts, there are too much parts".to_string()));
    }
    
    let expense_insert_id = match add_detail_expense_service(&insertable_detail_expense.expense, &insertable_detail_expense.contributors, &insertable_detail_expense.participants, &refunds_list) {
        Ok(detail_expense_id) => detail_expense_id,
        Err(e) => return Err(e),
    };

    match get_expense_by_id(group_id, &expense_insert_id, &authenticated_user){
        Ok(expense) => Ok(expense),
        Err(e) => Err(e),
    }
}

pub fn is_contributor_contain_duplicate(contributors: &Vec<InsertableContributor>) -> bool {
    let mut ids = vec![];
    for contributor in contributors {
        if ids.contains(&contributor.groups_users_id) {
            return true;
        }
        ids.push(contributor.groups_users_id);
    }
    false
}

pub fn calculate_amount(contributed: &f64, participated: &f64) -> f64 {
    if contributed > participated {
        return *participated;
    } else if contributed < participated {
        return *contributed;
    }
    0.0
}

pub fn participant_is_contributor(participant: &InsertableParticipant, contributors: &Vec<InsertableContributor>) -> Result<f64, bool> {
    for contributor in contributors {
        if contributor.groups_users_id == participant.groups_users_id {
            return Ok(contributor.amount_contributed)
        }
    }
    return Err(false);
}

/**
    * Ajoute une dépense détaillée avec ses contributeurs, participants et remboursements.
    *
    * Cette fonction insère une dépense, ses contributeurs, participants et remboursements dans la base de données.
    * Elle utilise une transaction pour garantir l'intégrité des données.
    *
    * @param expense La dépense à insérer.
    * @param contributors Les contributeurs associés à la dépense.
    * @param participants Les participants associés à la dépense.
    * @param refunds Les remboursements associés à la dépense pour les utilisateur qui sont à la fois contributeur et participant /!\ les id du remboursement doivent être les id user_groups et id_participant et id_contributeur (ces id seront changé au sein de la fonction).
    * @return Un objet DetailExpense contenant les informations de la dépense insérée.
    */
pub fn add_detail_expense_service(expense: &InsertableExpense, contributors: &Vec<InsertableContributor>, participants: &Vec<InsertableParticipant>, refunds: &Vec<InsertableRefund>) -> Result<i32, (Status, String)>{
    let mut conn = establish_connection();

    let transaction_result = conn.transaction::<_, diesel::result::Error, _>(|conn| {
        // Insertion de la dépense seule et recuperation de l'ID
        let expenses_id = expense_repository::insert_expense_repository(expense.clone(), conn).map_err(|_| diesel::result::Error::RollbackTransaction)?.id;

        // Insertion de l'id de la dépense dans les contributeurs et participants
        let mut contributors = contributors.clone();
        let mut participants = participants.clone();
        modify_expense_id_in_contributors_participants(expenses_id, &mut contributors, &mut participants);

        // Insertion des contributeurs
        let contributors = contributor_repository::insert_all_contributors(contributors, conn).map_err(|_| diesel::result::Error::RollbackTransaction)?;

        // Insertion des participants
        let participants = participant_repository::insert_all_participants(participants, conn).map_err(|_| diesel::result::Error::RollbackTransaction)?;

        // Modification des ID des contributeurs et participants pour les remboursements

        let mut refunds = refunds.clone();

        for refund in &mut refunds {
            for contributor in contributors.clone() {
                if refund.contributors_id == contributor.groups_users_id { 
                    refund.contributors_id = contributor.id;
                }
            }
            
            for participant in participants.clone() {
                if refund.participants_id == participant.groups_users_id { 
                    refund.participants_id = participant.id;
                }
            }
        }

        // Insertion des remboursements
        refund_repository::insert_all_refunds(&mut refunds, conn).map_err(|_| diesel::result::Error::RollbackTransaction)?;
        
        Ok(expenses_id)
    });

    match transaction_result {
        Ok(detail_expense) => Ok(detail_expense),
        Err(e) => Err((Status::InternalServerError, format!("Erreur Diesel: {e}"))),
    }
}

/**
modifie l'id de la dépense dans contributeurs, participants et remboursements
*/
pub fn modify_expense_id_in_contributors_participants(expense_id: i32, contributors: &mut Vec<InsertableContributor>, participants: &mut Vec<InsertableParticipant>) {
    for contributor in contributors {
        contributor.expenses_id = Option::from(expense_id);
    }
    for participant in participants {
        participant.expenses_id = Option::from(expense_id);
    }
}