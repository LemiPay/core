use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use diesel::{AsChangeset, Insertable, Queryable, Selectable};
use diesel_derive_enum::DbEnum;
use uuid::Uuid;

use crate::{domain::expense::ExpenseStatus, infrastructure::db::schema};

#[derive(Debug, DbEnum, Clone, Copy, PartialEq, Eq)]
#[db_enum(existing_type_path = "crate::infrastructure::db::schema::sql_types::ExpenseStatus")]
pub enum ExpenseStatusModel {
    Created,
    Verified,
    Updated,
    Deleted,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::expense)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ExpenseModel {
    pub expense_id: Uuid,
    pub user_id: Uuid,
    pub currency_id: Uuid,
    pub group_id: Uuid,
    pub description: Option<String>,
    pub amount: BigDecimal,
    pub status: ExpenseStatusModel,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = schema::expense)]
pub struct NewExpenseModel {
    pub user_id: Uuid,
    pub group_id: Uuid,
    pub currency_id: Uuid,
    pub amount: BigDecimal,
    pub description: Option<String>,
}

#[derive(AsChangeset)]
#[diesel(table_name = schema::expense)]
pub struct ExpenseUpdateModel {
    pub currency_id: Option<Uuid>,
    pub amount: Option<BigDecimal>,
    pub description: Option<String>,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::expense_participant)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ExpenseParticipantModel {
    pub expense_id: Uuid,
    pub user_id: Uuid,
    pub amount: BigDecimal,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = schema::expense_participant)]
pub struct NewExpenseParticipantModel {
    pub expense_id: Uuid,
    pub user_id: Uuid,
    pub amount: BigDecimal,
}

impl From<ExpenseStatusModel> for ExpenseStatus {
    fn from(value: ExpenseStatusModel) -> Self {
        match value {
            ExpenseStatusModel::Created => ExpenseStatus::Created,
            ExpenseStatusModel::Verified => ExpenseStatus::Verified,
            ExpenseStatusModel::Updated => ExpenseStatus::Updated,
            ExpenseStatusModel::Deleted => ExpenseStatus::Deleted,
        }
    }
}

impl From<ExpenseStatus> for ExpenseStatusModel {
    fn from(value: ExpenseStatus) -> Self {
        match value {
            ExpenseStatus::Created => ExpenseStatusModel::Created,
            ExpenseStatus::Verified => ExpenseStatusModel::Verified,
            ExpenseStatus::Updated => ExpenseStatusModel::Updated,
            ExpenseStatus::Deleted => ExpenseStatusModel::Deleted,
        }
    }
}
