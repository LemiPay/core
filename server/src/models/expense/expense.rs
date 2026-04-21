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
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Expense {
    pub expense_id: Uuid,
    pub user_id: Uuid,
    pub currency_id: Uuid,
    pub group_id: Uuid,
    pub description: Option<String>,
    pub amount: BigDecimal,
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

#[derive(AsChangeset)]
#[diesel(table_name = expense)]
pub struct ExpenseUpdate {
    pub currency_id: Option<Uuid>,
    pub amount: Option<BigDecimal>,
    pub description: Option<String>,
}

#[derive(Queryable, Serialize, Selectable)]
#[diesel(table_name = expense_participant)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[allow(dead_code)] // TODO: remove after implemented
pub struct ExpenseParticipant {
    pub expense_id: Uuid,
    pub user_id: Uuid,
    pub amount: BigDecimal,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = expense_participant)]
pub struct NewExpenseParticipant {
    pub expense_id: Uuid,
    pub user_id: Uuid,
    pub amount: BigDecimal,
}
