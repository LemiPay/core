use chrono::Utc;
use diesel::{
    BoolExpressionMethods, ExpressionMethods, OptionalExtension, PgTextExpressionMethods, QueryDsl,
    RunQueryDsl, SelectableHelper,
};
use uuid::Uuid;

use crate::application::{
    common::repo_error::RepoError,
    friend::dto::{FriendDetails, NewFriend, UserSearchResult},
    friend::traits::repository::FriendRepository,
};
use crate::domain::friend::FriendStatus;
use crate::infrastructure::db::{
    models::{
        friend::{FriendModel, FriendUpdateModel, NewFriendModel},
        user::UserModel,
    },
    pool::{DbConn, DbPool},
    schema,
};

pub struct DieselFriendRepository {
    db: DbPool,
}

impl DieselFriendRepository {
    pub fn new(db: DbPool) -> Self {
        Self { db }
    }

    fn get_conn(&self) -> Result<DbConn, RepoError> {
        self.db.get().map_err(|_| RepoError::Connection)
    }

    fn enrich_details(
        conn: &mut DbConn,
        models: Vec<FriendModel>,
        user_id: Uuid,
    ) -> Result<Vec<FriendDetails>, RepoError> {
        models
            .into_iter()
            .map(|m| {
                let other_id = if m.requester_id == user_id {
                    m.addressee_id
                } else {
                    m.requester_id
                };
                let user = schema::user::table
                    .filter(schema::user::id.eq(other_id))
                    .select(UserModel::as_select())
                    .first::<UserModel>(conn)
                    .optional()
                    .map_err(|_| RepoError::Query)?;

                Ok(FriendDetails {
                    user_id: other_id,
                    name: user.as_ref().map(|u| u.name.clone()).unwrap_or_default(),
                    email: user.as_ref().map(|u| u.email.clone()).unwrap_or_default(),
                    status: FriendStatus::from_str(&m.status).unwrap_or(FriendStatus::Pending),
                    created_at: m.created_at,
                })
            })
            .collect()
    }
}

impl FriendRepository for DieselFriendRepository {
    fn find_with_details(
        &self,
        requester_id: Uuid,
        addressee_id: Uuid,
    ) -> Result<Option<FriendDetails>, RepoError> {
        let mut conn = self.get_conn()?;
        let row = schema::friend::table
            .filter(schema::friend::requester_id.eq(requester_id))
            .filter(schema::friend::addressee_id.eq(addressee_id))
            .select(FriendModel::as_select())
            .first::<FriendModel>(&mut conn)
            .optional()
            .map_err(|_| RepoError::Query)?;

        match row {
            Some(m) => {
                let mut results = Self::enrich_details(&mut conn, vec![m], requester_id)?;
                Ok(results.pop())
            }
            None => Ok(None),
        }
    }

    fn find_relationship(
        &self,
        requester_id: Uuid,
        addressee_id: Uuid,
    ) -> Result<Option<(FriendStatus, Uuid, Uuid)>, RepoError> {
        let mut conn = self.get_conn()?;
        let row = schema::friend::table
            .filter(schema::friend::requester_id.eq(requester_id))
            .filter(schema::friend::addressee_id.eq(addressee_id))
            .select(FriendModel::as_select())
            .first::<FriendModel>(&mut conn)
            .optional()
            .map_err(|_| RepoError::Query)?;

        Ok(row.map(|m| {
            (
                FriendStatus::from_str(&m.status).unwrap_or(FriendStatus::Pending),
                m.requester_id,
                m.addressee_id,
            )
        }))
    }

    fn find_relationship_bidirectional(
        &self,
        user_id_1: Uuid,
        user_id_2: Uuid,
    ) -> Result<Option<(FriendStatus, Uuid, Uuid)>, RepoError> {
        let mut conn = self.get_conn()?;
        let row = schema::friend::table
            .filter(
                (schema::friend::requester_id
                    .eq(user_id_1)
                    .and(schema::friend::addressee_id.eq(user_id_2)))
                .or(schema::friend::requester_id
                    .eq(user_id_2)
                    .and(schema::friend::addressee_id.eq(user_id_1))),
            )
            .select(FriendModel::as_select())
            .first::<FriendModel>(&mut conn)
            .optional()
            .map_err(|_| RepoError::Query)?;

        Ok(row.map(|m| {
            (
                FriendStatus::from_str(&m.status).unwrap_or(FriendStatus::Pending),
                m.requester_id,
                m.addressee_id,
            )
        }))
    }

    fn insert(&self, new_friend: NewFriend) -> Result<(), RepoError> {
        let mut conn = self.get_conn()?;
        diesel::insert_into(schema::friend::table)
            .values(&NewFriendModel {
                requester_id: new_friend.requester_id,
                addressee_id: new_friend.addressee_id,
                status: new_friend.status.as_str().to_string(),
            })
            .execute(&mut conn)
            .map_err(|_| RepoError::Insert)?;
        Ok(())
    }

    fn update_status(
        &self,
        requester_id: Uuid,
        addressee_id: Uuid,
        status: FriendStatus,
    ) -> Result<(), RepoError> {
        let mut conn = self.get_conn()?;
        diesel::update(
            schema::friend::table
                .filter(schema::friend::requester_id.eq(requester_id))
                .filter(schema::friend::addressee_id.eq(addressee_id)),
        )
        .set(&FriendUpdateModel {
            status: status.as_str().to_string(),
            updated_at: Utc::now().naive_utc(),
        })
        .execute(&mut conn)
        .map_err(|_| RepoError::Insert)?;
        Ok(())
    }

    fn list_by_user_and_status(
        &self,
        user_id: Uuid,
        status: FriendStatus,
    ) -> Result<Vec<FriendDetails>, RepoError> {
        let mut conn = self.get_conn()?;
        let status_str = status.as_str().to_string();
        let rows = schema::friend::table
            .filter(
                (schema::friend::requester_id
                    .eq(user_id)
                    .or(schema::friend::addressee_id.eq(user_id)))
                .and(schema::friend::status.eq(&status_str)),
            )
            .select(FriendModel::as_select())
            .load::<FriendModel>(&mut conn)
            .map_err(|_| RepoError::Query)?;

        Self::enrich_details(&mut conn, rows, user_id)
    }

    fn list_friends(&self, user_id: Uuid) -> Result<Vec<FriendDetails>, RepoError> {
        self.list_by_user_and_status(user_id, FriendStatus::Accepted)
    }

    fn list_received_requests(&self, user_id: Uuid) -> Result<Vec<FriendDetails>, RepoError> {
        let mut conn = self.get_conn()?;
        let rows = schema::friend::table
            .filter(schema::friend::addressee_id.eq(user_id))
            .filter(schema::friend::status.eq("pending"))
            .select(FriendModel::as_select())
            .load::<FriendModel>(&mut conn)
            .map_err(|_| RepoError::Query)?;

        Self::enrich_details(&mut conn, rows, user_id)
    }

    fn list_sent_requests(&self, user_id: Uuid) -> Result<Vec<FriendDetails>, RepoError> {
        let mut conn = self.get_conn()?;
        let rows = schema::friend::table
            .filter(schema::friend::requester_id.eq(user_id))
            .filter(schema::friend::status.eq("pending"))
            .select(FriendModel::as_select())
            .load::<FriendModel>(&mut conn)
            .map_err(|_| RepoError::Query)?;

        Self::enrich_details(&mut conn, rows, user_id)
    }

    fn search_users(
        &self,
        current_user_id: Uuid,
        query: &str,
    ) -> Result<Vec<UserSearchResult>, RepoError> {
        let mut conn = self.get_conn()?;
        let pattern = format!("%{}%", query);

        let users = schema::user::table
            .filter(
                schema::user::name
                    .ilike(&pattern)
                    .or(schema::user::email.ilike(&pattern)),
            )
            .filter(schema::user::id.ne(current_user_id))
            .select(UserModel::as_select())
            .load::<UserModel>(&mut conn)
            .map_err(|_| RepoError::Query)?;

        let friend_ids: Vec<Uuid> = {
            let friend_rows = schema::friend::table
                .filter(
                    (schema::friend::requester_id
                        .eq(current_user_id)
                        .or(schema::friend::addressee_id.eq(current_user_id)))
                    .and(schema::friend::status.eq("accepted")),
                )
                .select(FriendModel::as_select())
                .load::<FriendModel>(&mut conn)
                .map_err(|_| RepoError::Query)?;

            friend_rows
                .into_iter()
                .map(|f| {
                    if f.requester_id == current_user_id {
                        f.addressee_id
                    } else {
                        f.requester_id
                    }
                })
                .collect()
        };

        let mut results: Vec<UserSearchResult> = users
            .into_iter()
            .map(|u| {
                let is_friend = friend_ids.contains(&u.id);
                UserSearchResult {
                    user_id: u.id,
                    name: u.name,
                    email: u.email,
                    is_friend,
                }
            })
            .collect();

        results.sort_by(|a, b| b.is_friend.cmp(&a.is_friend).then(a.name.cmp(&b.name)));

        Ok(results)
    }

    fn delete(&self, requester_id: Uuid, addressee_id: Uuid) -> Result<(), RepoError> {
        let mut conn = self.get_conn()?;
        diesel::delete(
            schema::friend::table
                .filter(schema::friend::requester_id.eq(requester_id))
                .filter(schema::friend::addressee_id.eq(addressee_id)),
        )
        .execute(&mut conn)
        .map_err(|_| RepoError::Query)?;
        Ok(())
    }
}
