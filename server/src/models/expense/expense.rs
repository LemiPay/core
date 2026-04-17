use crate::schema::group::description;
use crate::schema::sql_types::ExpenseStatus;
use crate::schema::{expense, expense_participant};
use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use diesel::{AsChangeset, Insertable, Queryable, Selectable};
use diesel_derive_enum::DbEnum;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, DbEnum, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
#[db_enum(existing_type_path = "crate::schema::sql_types::ExpenseStatus")]
pub enum MyExpenseStatus {
    Created,
    Verified,
    Updated,
    Deleted,
}

#[derive(Queryable, Serialize, Selectable)]
#[diesel(table_name = expense)]
pub struct Expense {
    pub expense_id: Uuid,
    pub user_id: Uuid,
    pub currency_id: Uuid,
    pub group_id: Uuid,
    pub amount: BigDecimal,
    pub description: Option<String>,
    pub status: MyExpenseStatus,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = expense)]
pub struct NewExpense {
    pub user_id: Uuid,
    pub group_id: Uuid,
    pub currency_id: Uuid,
    pub amount: BigDecimal,
    pub description: Option<String>,
}
