use axum::{Json, extract::State};

use crate::application::group::list_user_groups::dto::ListUserGroupsInput;

use crate::interfaces::http::{
    auth::extractor::AuthUser, error::AppError, group::dto::ListUserGroupsResponse,
};

use crate::setup::state::SharedState;

pub async fn list_user_groups(
    State(state): State<SharedState>,
    user: AuthUser,
) -> Result<Json<Vec<ListUserGroupsResponse>>, AppError> {
    let output = state
        .group_service
        .list_user_groups
        .execute(ListUserGroupsInput {
            user_id: user.user_id,
        })
        .map_err(AppError::from)?;

    Ok(Json(
        output
            .groups
            .into_iter()
            .map(|g| ListUserGroupsResponse {
                user_id: g.user_id,
                group_id: g.group_id,
                role: g.role,
                group_name: g.group_name,
                group_description: g.group_description,
                status: g.status,
            })
            .collect(),
    ))
}
