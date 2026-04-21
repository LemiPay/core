use bigdecimal::BigDecimal;
use diesel::prelude::*;
use uuid::Uuid;

use crate::data::database::Db;
use crate::data::error::DbError;
use crate::models::expense::{
    Expense, ExpenseUpdate, MyExpenseStatus, NewExpense, NewExpenseParticipant,
};
use crate::repositories::traits::expense_repo::ExpenseRepository;
use crate::schema::{expense, expense_participant};

pub struct DieselExpenseRepository {
    db: Db,
}

impl DieselExpenseRepository {
    pub fn new(db: Db) -> Self {
        Self { db }
    }
}

impl ExpenseRepository for DieselExpenseRepository {
    fn find_by_id(&self, expense_id: Uuid) -> Result<Option<Expense>, DbError> {
        let mut conn = self.db.get_conn()?;
        let result = expense::table
            .filter(expense::expense_id.eq(expense_id))
            .select(Expense::as_select())
            .first::<Expense>(&mut conn)
            .optional()?;
        Ok(result)
    }

    fn find_by_group(&self, group_id: Uuid) -> Result<Vec<Expense>, DbError> {
        let mut conn = self.db.get_conn()?;
        let result = expense::table
            .filter(expense::group_id.eq(group_id))
            .filter(expense::status.ne(MyExpenseStatus::Deleted))
            .order(expense::updated_at.asc())
            .select(Expense::as_select())
            .get_results::<Expense>(&mut conn)?;
        Ok(result)
    }

    fn create(
        &self,
        new_expense: NewExpense,
        participants: Vec<(Uuid, BigDecimal)>,
    ) -> Result<Expense, DbError> {
        let mut conn = self.db.get_conn()?;

        let result = conn.transaction::<Expense, DbError, _>(|conn| {
            let inserted = diesel::insert_into(expense::table)
                .values(&new_expense)
                .returning(Expense::as_returning())
                .get_result(conn)?;

            let rows: Vec<NewExpenseParticipant> = participants
                .into_iter()
                .map(|(user_id, amount)| NewExpenseParticipant {
                    expense_id: inserted.expense_id,
                    user_id,
                    amount,
                })
                .collect();

            diesel::insert_into(expense_participant::table)
                .values(&rows)
                .execute(conn)?;

            Ok(inserted)
        })?;

        Ok(result)
    }

    fn update(
        &self,
        expense_id: Uuid,
        update: ExpenseUpdate,
        participants: Option<Vec<(Uuid, BigDecimal)>>,
    ) -> Result<Expense, DbError> {
        let mut conn = self.db.get_conn()?;

        let result = conn.transaction::<Expense, DbError, _>(|conn| {
            let updated = diesel::update(expense::table.filter(expense::expense_id.eq(expense_id)))
                .set((update, expense::status.eq(MyExpenseStatus::Updated)))
                .returning(Expense::as_returning())
                .get_result(conn)?;

            if let Some(new_participants) = participants {
                diesel::delete(
                    expense_participant::table
                        .filter(expense_participant::expense_id.eq(expense_id)),
                )
                .execute(conn)?;

                let rows: Vec<NewExpenseParticipant> = new_participants
                    .into_iter()
                    .map(|(user_id, amount)| NewExpenseParticipant {
                        expense_id,
                        user_id,
                        amount,
                    })
                    .collect();

                diesel::insert_into(expense_participant::table)
                    .values(&rows)
                    .execute(conn)?;
            }

            Ok(updated)
        })?;

        Ok(result)
    }

    fn soft_delete(&self, expense_id: Uuid) -> Result<Expense, DbError> {
        let mut conn = self.db.get_conn()?;
        let result = diesel::update(expense::table.filter(expense::expense_id.eq(expense_id)))
            .set(expense::status.eq(MyExpenseStatus::Deleted))
            .returning(Expense::as_returning())
            .get_result::<Expense>(&mut conn)?;
        Ok(result)
    }
}
