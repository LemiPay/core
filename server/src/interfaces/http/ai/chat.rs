use axum::{Json, extract::State};

use crate::application::ai::context;
use crate::application::group::list_user_groups::dto::ListUserGroupsInput;
use crate::domain::group::GroupId;
use crate::interfaces::http::ai::dto::{AskRequest, AskResponse, ExplainRequest, ExplainResponse};
use crate::interfaces::http::auth::extractor::AuthUser;
use crate::interfaces::http::error::AppError;
use crate::setup::state::SharedState;

pub async fn ask(
    State(state): State<SharedState>,
    user: AuthUser,
    Json(body): Json<AskRequest>,
) -> Result<Json<AskResponse>, AppError> {
    let system_prompt = context::assistant_system_prompt();
    let context = build_user_context(&state, &user).await?;

    let answer = state
        .ai_service
        .ask(&system_prompt, &context, &body.question)
        .await
        .map_err(AppError::from)?;

    Ok(Json(AskResponse { answer }))
}

pub async fn explain(
    State(state): State<SharedState>,
    Json(body): Json<ExplainRequest>,
) -> Result<Json<ExplainResponse>, AppError> {
    let explanation = state
        .ai_service
        .explain(&body.concept)
        .await
        .map_err(AppError::from)?;

    Ok(Json(ExplainResponse { explanation }))
}

async fn build_user_context(state: &SharedState, user: &AuthUser) -> Result<String, AppError> {
    let groups_output = state
        .group_service
        .list_user_groups
        .execute(ListUserGroupsInput {
            user_id: user.user_id,
        })
        .map_err(AppError::from)?;

    let mut group_ids = Vec::new();
    let mut balances = Vec::new();
    for group in &groups_output.groups {
        if let Ok(balance) = state.balances_service.get_balances(GroupId(group.group_id)) {
            group_ids.push(group.group_id.to_string());
            balances.push(balance);
        }
    }

    let balance_refs: Vec<_> = balances
        .iter()
        .zip(group_ids.iter())
        .map(|(b, id)| (b, id.as_str()))
        .collect();

    Ok(crate::application::ai::formatter::format_user_groups(
        &groups_output.groups,
        &balance_refs,
    ))
}
