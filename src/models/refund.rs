use chrono::NaiveDateTime;
use diesel::{AsChangeset, Insertable, Queryable, Selectable, QueryableByName};
use rocket::serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Serialize, Clone, AsChangeset, Deserialize)]
#[diesel(table_name = crate::schema::refunds)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Refund {
    pub id: i32,
    pub amount: f64,
    pub status: String,
    pub contributors_id: i32,
    pub participants_id: i32,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}


// Struct pour les INSERTS sans les champs auto-générés
#[derive(Insertable, Deserialize, Clone)]
#[diesel(table_name = crate::schema::refunds)]
pub struct InsertableRefund {
    pub amount: f64,
    pub status: String,
    pub contributors_id: i32,
    pub participants_id: i32,
    pub created_at: Option<NaiveDateTime>,
}
#[derive(Deserialize, Clone, Serialize)]
pub struct BalanceGroup{
    pub group_id: i32,
    pub total_cost_group: f64,
    pub total_cost_user: f64,
    pub total_contributed_user: f64,
    pub balance_user: f64,
}

#[derive(QueryableByName)]
pub struct TotalCost {
    #[sql_type = "diesel::sql_types::Nullable<diesel::sql_types::Double>"]
    pub total_cost: Option<f64>,
}

#[derive(QueryableByName)]
pub struct TotalRefundAmount {
    #[sql_type = "diesel::sql_types::Nullable<diesel::sql_types::Double>"]
    pub total_refund: Option<f64>,
}

#[derive(QueryableByName)]
pub struct TotalParticipated {
    #[sql_type = "diesel::sql_types::Nullable<diesel::sql_types::Double>"]
    pub total_participated: Option<f64>,
}

#[derive(QueryableByName)]
pub struct TotalContributed {
    #[sql_type = "diesel::sql_types::Nullable<diesel::sql_types::Double>"]
    pub total_contributed: Option<f64>,
}