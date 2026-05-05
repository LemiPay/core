use bigdecimal::BigDecimal;
use diesel::{
    Connection, ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl, SelectableHelper,
};
use uuid::Uuid;

use crate::{
    application::{
        common::repo_error::RepoError,
        expense::{
            dto::{ExpenseDetails, ExpenseUpdate, NewExpense},
            traits::repository::ExpenseRepository,
        },
    },
    domain::expense::ExpenseStatus,
    infrastructure::db::{
        models::expense::{
            ExpenseModel, ExpenseParticipantModel, ExpenseStatusModel, ExpenseUpdateModel,
            NewExpenseModel, NewExpenseParticipantModel,
        },
        pool::{DbConn, DbPool},
        schema,
    },
};

pub struct DieselExpenseRepository {
    db: DbPool,
}

impl DieselExpenseRepository {
    pub fn new(db: DbPool) -> Self {
        Self { db }
    }

    fn get_conn(&self) -> Result<DbConn, RepoError> {
        self.db.get().map_err(|_| RepoError::Connection)
    }

    fn to_details(model: ExpenseModel) -> ExpenseDetails {
        ExpenseDetails {
            expense_id: model.expense_id,
            user_id: model.user_id,
            currency_id: model.currency_id,
            group_id: model.group_id,
            description: model.description,
            amount: model.amount,
            status: model.status.into(),
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}

impl ExpenseRepository for DieselExpenseRepository {
    fn find_by_id(&self, expense_id: Uuid) -> Result<Option<ExpenseDetails>, RepoError> {
        let mut conn = self.get_conn()?;
        let row = schema::expense::table
            .filter(schema::expense::expense_id.eq(expense_id))
            .select(ExpenseModel::as_select())
            .first::<ExpenseModel>(&mut conn)
            .optional()
            .map_err(|_| RepoError::Query)?;
        Ok(row.map(Self::to_details))
    }

    fn find_by_group(&self, group_id: Uuid) -> Result<Vec<ExpenseDetails>, RepoError> {
        let mut conn = self.get_conn()?;
        let rows = schema::expense::table
            .filter(schema::expense::group_id.eq(group_id))
            .filter(schema::expense::status.ne(ExpenseStatusModel::Deleted))
            .order(schema::expense::updated_at.asc())
            .select(ExpenseModel::as_select())
            .load::<ExpenseModel>(&mut conn)
            .map_err(|_| RepoError::Query)?;
        Ok(rows.into_iter().map(Self::to_details).collect())
    }

    fn create(
        &self,
        new_expense: NewExpense,
        participants: Vec<(Uuid, BigDecimal)>,
    ) -> Result<ExpenseDetails, RepoError> {
        let mut conn = self.get_conn()?;
        conn.transaction::<ExpenseDetails, diesel::result::Error, _>(|tx| {
            let inserted = diesel::insert_into(schema::expense::table)
                .values(&NewExpenseModel {
                    user_id: new_expense.user_id,
                    group_id: new_expense.group_id,
                    currency_id: new_expense.currency_id,
                    amount: new_expense.amount,
                    description: new_expense.description,
                })
                .returning(ExpenseModel::as_returning())
                .get_result::<ExpenseModel>(tx)?;

            let rows: Vec<NewExpenseParticipantModel> = participants
                .into_iter()
                .map(|(user_id, amount)| NewExpenseParticipantModel {
                    expense_id: inserted.expense_id,
                    user_id,
                    amount,
                })
                .collect();

            diesel::insert_into(schema::expense_participant::table)
                .values(&rows)
                .execute(tx)?;

            Ok(Self::to_details(inserted))
        })
        .map_err(|_| RepoError::Insert)
    }

    fn update(
        &self,
        expense_id: Uuid,
        update: ExpenseUpdate,
        participants: Option<Vec<(Uuid, BigDecimal)>>,
    ) -> Result<ExpenseDetails, RepoError> {
        let mut conn = self.get_conn()?;
        conn.transaction::<ExpenseDetails, diesel::result::Error, _>(|tx| {
            let updated = diesel::update(
                schema::expense::table.filter(schema::expense::expense_id.eq(expense_id)),
            )
            .set((
                ExpenseUpdateModel {
                    currency_id: update.currency_id,
                    amount: update.amount,
                    description: update.description,
                },
                schema::expense::status.eq(ExpenseStatusModel::from(ExpenseStatus::Updated)),
            ))
            .returning(ExpenseModel::as_returning())
            .get_result::<ExpenseModel>(tx)?;

            if let Some(participants) = participants {
                let existing_rows = schema::expense_participant::table
                    .filter(schema::expense_participant::expense_id.eq(expense_id))
                    .select(ExpenseParticipantModel::as_select())
                    .load::<ExpenseParticipantModel>(tx)?;

                if !existing_rows.is_empty() {
                    diesel::delete(
                        schema::expense_participant::table
                            .filter(schema::expense_participant::expense_id.eq(expense_id)),
                    )
                    .execute(tx)?;
                }

                let rows: Vec<NewExpenseParticipantModel> = participants
                    .into_iter()
                    .map(|(user_id, amount)| NewExpenseParticipantModel {
                        expense_id,
                        user_id,
                        amount,
                    })
                    .collect();

                diesel::insert_into(schema::expense_participant::table)
                    .values(&rows)
                    .execute(tx)?;
            }

            Ok(Self::to_details(updated))
        })
        .map_err(|_| RepoError::Insert)
    }

    fn soft_delete(&self, expense_id: Uuid) -> Result<ExpenseDetails, RepoError> {
        let mut conn = self.get_conn()?;
        let row = diesel::update(
            schema::expense::table.filter(schema::expense::expense_id.eq(expense_id)),
        )
        .set(schema::expense::status.eq(ExpenseStatusModel::Deleted))
        .returning(ExpenseModel::as_returning())
        .get_result::<ExpenseModel>(&mut conn)
        .map_err(|_| RepoError::Insert)?;
        Ok(Self::to_details(row))
    }
}
