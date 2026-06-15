use diesel::RunQueryDsl;

use crate::domain::group::GroupId;
use crate::domain::user::UserId;
use crate::infrastructure::db::models::notifications::NewNotificationRecord;
use crate::infrastructure::db::pool::DbPool;
use crate::infrastructure::db::schema;
use crate::infrastructure::notification::persistent_service::{
    PersistentNotificationError, PersistentNotificationService,
};

pub struct DbPersistentNotificationService {
    db: DbPool,
}

impl DbPersistentNotificationService {
    pub fn new(db: DbPool) -> Self {
        Self { db }
    }

    fn insert(
        &self,
        user_id: UserId,
        group_id: Option<GroupId>,
        event_name: &str,
        group_name: Option<&str>,
    ) -> Result<(), PersistentNotificationError> {
        let mut conn = self
            .db
            .get()
            .map_err(|_| PersistentNotificationError::Internal)?;

        let record = NewNotificationRecord {
            user_id: user_id.into(),
            group_id: group_id.map(Into::into),
            event_name: event_name.to_string(),
            group_name: group_name.map(|s| s.to_string()),
            read: false,
        };

        diesel::insert_into(schema::notification::table)
            .values(&record)
            .execute(&mut conn)
            .map_err(|_| PersistentNotificationError::SaveFailed)?;

        Ok(())
    }
}

impl PersistentNotificationService for DbPersistentNotificationService {
    fn example(&self, user_id: UserId) -> Result<(), PersistentNotificationError> {
        self.insert(user_id, None, "example", None)
    }

    fn save_welcome_notification(
        &self,
        user_id: UserId,
        _name: &str,
    ) -> Result<(), PersistentNotificationError> {
        self.insert(user_id, None, "welcome", None)
    }

    fn save_login_alert(
        &self,
        user_id: UserId,
        _name: &str,
    ) -> Result<(), PersistentNotificationError> {
        self.insert(user_id, None, "login_alert", None)
    }

    fn save_withdraw_proposal_created(
        &self,
        user_id: UserId,
        group_id: GroupId,
        group_name: &str,
    ) -> Result<(), PersistentNotificationError> {
        self.insert(
            user_id,
            Some(group_id),
            "withdraw_proposal_created",
            Some(group_name),
        )
    }

    fn save_proposal_approved(
        &self,
        user_id: UserId,
        group_id: GroupId,
        group_name: &str,
    ) -> Result<(), PersistentNotificationError> {
        self.insert(
            user_id,
            Some(group_id),
            "proposal_approved",
            Some(group_name),
        )
    }
    fn save_proposal_rejected(
        &self,
        user_id: UserId,
        group_id: GroupId,
        group_name: &str,
    ) -> Result<(), PersistentNotificationError> {
        self.insert(
            user_id,
            Some(group_id),
            "proposal_rejected",
            Some(group_name),
        )
    }

    fn save_proposal_executed(
        &self,
        user_id: UserId,
        group_id: GroupId,
        group_name: &str,
    ) -> Result<(), PersistentNotificationError> {
        self.insert(
            user_id,
            Some(group_id),
            "proposal_executed",
            Some(group_name),
        )
    }

    fn save_new_member_added(
        &self,
        user_id: UserId,
        group_id: GroupId,
        group_name: &str,
    ) -> Result<(), PersistentNotificationError> {
        self.insert(
            user_id,
            Some(group_id),
            "new_member_added",
            Some(group_name),
        )
    }

    fn save_fund_round_created(
        &self,
        user_id: UserId,
        group_id: GroupId,
        group_name: &str,
    ) -> Result<(), PersistentNotificationError> {
        self.insert(
            user_id,
            Some(group_id),
            "fund_round_created",
            Some(group_name),
        )
    }

    fn save_investment_created(
        &self,
        user_id: UserId,
        group_id: GroupId,
        group_name: &str,
    ) -> Result<(), PersistentNotificationError> {
        self.insert(
            user_id,
            Some(group_id),
            "investment_created",
            Some(group_name),
        )
    }

    fn save_investment_matured(
        &self,
        user_id: UserId,
        group_id: GroupId,
        group_name: &str,
    ) -> Result<(), PersistentNotificationError> {
        self.insert(
            user_id,
            Some(group_id),
            "investment_matured",
            Some(group_name),
        )
    }

    fn save_expense_created(
        &self,
        user_id: UserId,
        group_id: GroupId,
        group_name: &str,
    ) -> Result<(), PersistentNotificationError> {
        self.insert(user_id, Some(group_id), "expense_created", Some(group_name))
    }
}
