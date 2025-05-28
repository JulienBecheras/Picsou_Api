use diesel::sql_types::{Timestamp, Int4, Float8, Varchar, Nullable};
use diesel::prelude::*;
use chrono::NaiveDateTime;
use rocket::serde::{Deserialize, Serialize};
use crate::models::contributor::{ContributorUserWithStatus};
use crate::models::group::{InsertableGroup};
use crate::models::participant::ParticipantUserWithStatus;
use crate::models::refund::Refund;

#[derive(Queryable, Selectable, Serialize, Clone, AsChangeset, Deserialize)]
#[diesel(table_name = crate::schema::expenses)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Expense {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub montant: f64,
    pub stock_parts: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

// Struct pour les INSERTS sans les champs auto-générés
#[derive(Insertable, Deserialize, Clone)]
#[diesel(table_name = crate::schema::expenses)]
pub struct InsertableExpense {
    pub name: String,
    pub description: String,
    pub montant: f64,
    pub stock_parts: i32,
    pub created_at: NaiveDateTime,
}

#[derive(Deserialize, Clone, Serialize)]
pub struct DetailExpense {
    pub group: InsertableGroup,
    pub contributors: Vec<ContributorUserWithStatus>,
    pub participants: Vec<ParticipantUserWithStatus>,
    pub expense: Expense,
    pub refunds: Vec<Refund>
}

#[derive(QueryableByName, Debug)]
pub struct DetailExpenseFlat {
    #[sql_type = "Int4"]
    pub group_id: i32,
    #[sql_type = "Varchar"]
    pub group_name: String,
    #[sql_type = "Varchar"]
    pub pict_ref: String,
    #[sql_type = "Timestamp"]
    pub created_at: NaiveDateTime,

    #[sql_type = "Nullable<Int4>"]
    pub expense_id: Option<i32>,
    #[sql_type = "Nullable<Varchar>"]
    pub expense_name: Option<String>,
    #[sql_type = "Nullable<Varchar>"]
    pub expense_description: Option<String>,
    #[sql_type = "Nullable<Float8>"]
    pub montant: Option<f64>,
    #[sql_type = "Nullable<Int4>"]
    pub stock_parts: Option<i32>,
    #[sql_type = "Nullable<Timestamp>"]
    pub expense_created_at: Option<NaiveDateTime>,
    #[sql_type = "Nullable<Timestamp>"]
    pub expense_updated_at: Option<NaiveDateTime>,

    #[sql_type = "Nullable<Int4>"]
    pub contributor_id: Option<i32>,
    #[sql_type = "Nullable<Float8>"]
    pub amount_contributed: Option<f64>,
    #[sql_type = "Nullable<Int4>"]
    pub contributor_group_user_id: Option<i32>,

    #[sql_type = "Nullable<Int4>"]
    pub participant_id: Option<i32>,
    #[sql_type = "Nullable<Float8>"]
    pub amount_participated: Option<f64>,
    #[sql_type = "Nullable<Int4>"]
    pub participant_group_user_id: Option<i32>,
    #[sql_type = "Nullable<Int4>"]
    pub part_number: Option<i32>,

    #[sql_type = "Nullable<Int4>"]
    pub refund_id: Option<i32>,
    #[sql_type = "Nullable<Float8>"]
    pub refund_amount: Option<f64>,
    #[sql_type = "Nullable<Varchar>"]
    pub refund_status: Option<String>,
    #[sql_type = "Nullable<Int4>"]
    pub refund_contributor_id: Option<i32>,
    #[sql_type = "Nullable<Int4>"]
    pub refund_participant_id: Option<i32>,
    #[sql_type = "Nullable<Timestamp>"]
    pub refund_created_at: Option<NaiveDateTime>,
    #[sql_type = "Nullable<Timestamp>"]
    pub refund_updated_at: Option<NaiveDateTime>,
}


