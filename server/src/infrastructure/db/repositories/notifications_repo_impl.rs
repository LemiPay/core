use diesel::{QueryDsl, RunQueryDsl, SelectableHelper, prelude::*};

use crate::application::{
    common::repo_error::RepoError, notifications::repository::NotificationRepository,
};

use crate::domain::{
    group::GroupId,
    notification::{
        GroupNotificationPreference, NotificationChannel, NotificationEvent,
        UserNotificationPreference,
    },
    user::UserId,
};

use crate::infrastructure::db::models::notifications::{
    GroupNotificationPreferenceModel, NewGroupNotificationPreference,
    NewUserNotificationPreference, UserNotificationPreferenceModel,
};
use crate::infrastructure::db::{
    models::notifications::{NotificationChannelModel, NotificationEventModel},
    pool::{DbConn, DbPool},
    schema,
};

pub struct DieselNotificationRepository {
    db: DbPool,
}

impl DieselNotificationRepository {
    pub fn new(db: DbPool) -> Self {
        Self { db }
    }

    fn get_conn(&self) -> Result<DbConn, RepoError> {
        self.db.get().map_err(|_| RepoError::Connection)
    }
}

impl NotificationRepository for DieselNotificationRepository {
    fn get_events(&self) -> Result<Vec<NotificationEvent>, RepoError> {
        let mut conn = self.get_conn()?;

        let rows = schema::notification_event::table
            .select(NotificationEventModel::as_select())
            .load::<NotificationEventModel>(&mut conn)
            .map_err(|_| RepoError::Query)?;

        Ok(rows
            .into_iter()
            .map(|row| NotificationEvent::from(row))
            .collect())
    }

    fn get_channels(&self) -> Result<Vec<NotificationChannel>, RepoError> {
        let mut conn = self.get_conn()?;

        let rows = schema::notification_channel::table
            .select(NotificationChannelModel::as_select())
            .load::<NotificationChannelModel>(&mut conn)
            .map_err(|_| RepoError::Query)?;

        Ok(rows
            .into_iter()
            .map(|row| NotificationChannel::from(row))
            .collect())
    }

    fn get_user_preferences(
        &self,
        user_id: UserId,
    ) -> Result<Vec<UserNotificationPreference>, RepoError> {
        let mut conn = self.get_conn()?;

        let rows = schema::user_notification_preference::table
            .filter(schema::user_notification_preference::user_id.eq(user_id.0))
            .load::<UserNotificationPreferenceModel>(&mut conn)
            .map_err(|_| RepoError::Query)?;

        Ok(rows
            .into_iter()
            .map(|row| UserNotificationPreference::from(row))
            .collect())
    }

    fn upsert_user_preference(
        &self,
        preference: UserNotificationPreference,
    ) -> Result<(), RepoError> {
        let mut conn = self.get_conn()?;

        let new_pref = NewUserNotificationPreference::from(preference);

        diesel::insert_into(schema::user_notification_preference::table)
            .values(&new_pref)
            .on_conflict((
                schema::user_notification_preference::user_id,
                schema::user_notification_preference::event_id,
                schema::user_notification_preference::channel_id,
            ))
            .do_update()
            .set(schema::user_notification_preference::enabled.eq(new_pref.enabled))
            .execute(&mut conn)
            .map_err(|_| RepoError::Query)?;

        Ok(())
    }

    fn get_group_preferences(
        &self,
        user_id: UserId,
        group_id: GroupId,
    ) -> Result<Vec<GroupNotificationPreference>, RepoError> {
        let mut conn = self.get_conn()?;

        let rows = schema::group_notification_preference::table
            .filter(schema::group_notification_preference::user_id.eq(user_id.0))
            .filter(schema::group_notification_preference::group_id.eq(group_id.0))
            .load::<GroupNotificationPreferenceModel>(&mut conn)
            .map_err(|_| RepoError::Query)?;

        Ok(rows
            .into_iter()
            .map(|row| GroupNotificationPreference::from(row))
            .collect())
    }

    fn upsert_group_preference(
        &self,
        preference: GroupNotificationPreference,
    ) -> Result<(), RepoError> {
        let mut conn = self.get_conn()?;

        let new_pref = NewGroupNotificationPreference::from(preference);

        diesel::insert_into(schema::group_notification_preference::table)
            .values(&new_pref)
            .on_conflict((
                schema::group_notification_preference::user_id,
                schema::group_notification_preference::group_id,
                schema::group_notification_preference::event_id,
                schema::group_notification_preference::channel_id,
            ))
            .do_update()
            .set(schema::group_notification_preference::enabled.eq(new_pref.enabled))
            .execute(&mut conn)
            .map_err(|_| RepoError::Query)?;

        Ok(())
    }
}
