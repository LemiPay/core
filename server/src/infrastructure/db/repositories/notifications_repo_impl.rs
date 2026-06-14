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

    fn initialize_defaults_for_user(&self, user_id: UserId) -> Result<(), RepoError> {
        let mut conn = self.get_conn()?;

        let events = self.get_events_in_conn(&mut conn)?;
        let channels = self.get_channels_in_conn(&mut conn)?;

        for event in &events {
            for channel in &channels {
                let new_pref = NewUserNotificationPreference {
                    user_id: user_id.0,
                    event_id: event.id.0, // since id_type wraps Uuid as .0
                    channel_id: channel.id.0,
                    enabled: true,
                };

                diesel::insert_into(schema::user_notification_preference::table)
                    .values(&new_pref)
                    .on_conflict((
                        schema::user_notification_preference::user_id,
                        schema::user_notification_preference::event_id,
                        schema::user_notification_preference::channel_id,
                    ))
                    .do_update()
                    .set(schema::user_notification_preference::enabled.eq(true))
                    .execute(&mut conn)
                    .map_err(|_| RepoError::Query)?;
            }
        }

        Ok(())
    }

    fn initialize_defaults_for_user_in_group(
        &self,
        user_id: UserId,
        group_id: GroupId,
    ) -> Result<(), RepoError> {
        let mut conn = self.get_conn()?;

        let events = self.get_events_in_conn(&mut conn)?;
        let channels = self.get_channels_in_conn(&mut conn)?;

        for event in &events {
            for channel in &channels {
                let new_pref = NewGroupNotificationPreference {
                    user_id: user_id.0,
                    group_id: group_id.0,
                    event_id: event.id.0,
                    channel_id: channel.id.0,
                    enabled: true,
                };

                diesel::insert_into(schema::group_notification_preference::table)
                    .values(&new_pref)
                    .on_conflict((
                        schema::group_notification_preference::user_id,
                        schema::group_notification_preference::group_id,
                        schema::group_notification_preference::event_id,
                        schema::group_notification_preference::channel_id,
                    ))
                    .do_update()
                    .set(schema::group_notification_preference::enabled.eq(true))
                    .execute(&mut conn)
                    .map_err(|_| RepoError::Query)?;
            }
        }

        Ok(())
    }
}

// Small private helpers to reuse events/channels inside an existing conn (avoid re-get_conn).
impl DieselNotificationRepository {
    fn get_events_in_conn(&self, conn: &mut DbConn) -> Result<Vec<NotificationEvent>, RepoError> {
        let rows = schema::notification_event::table
            .select(NotificationEventModel::as_select())
            .load::<NotificationEventModel>(conn)
            .map_err(|_| RepoError::Query)?;

        Ok(rows.into_iter().map(NotificationEvent::from).collect())
    }

    fn get_channels_in_conn(
        &self,
        conn: &mut DbConn,
    ) -> Result<Vec<NotificationChannel>, RepoError> {
        let rows = schema::notification_channel::table
            .select(NotificationChannelModel::as_select())
            .load::<NotificationChannelModel>(conn)
            .map_err(|_| RepoError::Query)?;

        Ok(rows.into_iter().map(NotificationChannel::from).collect())
    }
}
