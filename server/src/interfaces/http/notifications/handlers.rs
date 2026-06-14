use axum::{
    Json,
    extract::{Path, State},
};
use uuid::Uuid;

use crate::application::notifications::repository::NotificationRepository;
use crate::domain::group::GroupId;
use crate::domain::notification::{GroupNotificationPreference, UserNotificationPreference};
use crate::interfaces::http::auth::extractor::AuthUser;
use crate::interfaces::http::error::AppError;
use crate::interfaces::http::notifications::dto::{
    ChannelResponse, EventResponse, GroupPreferenceResponse, PreferenceResponse,
    UpsertPreferenceRequest,
};
use crate::setup::state::SharedState;

// =========================
// Metadata (events / channels)
// =========================

pub async fn get_events(
    State(state): State<SharedState>,
) -> Result<Json<Vec<EventResponse>>, AppError> {
    let events = state
        .notification_repo
        .get_events()
        .map_err(|_| AppError::Internal)?;
    let resp = events
        .into_iter()
        .map(|e| EventResponse {
            id: e.id.to_string(),
            name: e.name,
        })
        .collect();
    Ok(Json(resp))
}

pub async fn get_channels(
    State(state): State<SharedState>,
) -> Result<Json<Vec<ChannelResponse>>, AppError> {
    let channels = state
        .notification_repo
        .get_channels()
        .map_err(|_| AppError::Internal)?;
    let resp = channels
        .into_iter()
        .map(|c| ChannelResponse {
            id: c.id.to_string(),
            name: c.name,
        })
        .collect();
    Ok(Json(resp))
}

// =========================
// User (global) preferences
// =========================

pub async fn get_my_preferences(
    State(state): State<SharedState>,
    user: AuthUser,
) -> Result<Json<Vec<PreferenceResponse>>, AppError> {
    let prefs = state
        .notification_repo
        .get_user_preferences(user.user_id)
        .map_err(|_| AppError::Internal)?;

    let body = prefs
        .into_iter()
        .map(|p| PreferenceResponse {
            user_id: p.user_id.to_string(),
            event_id: p.event_id.to_string(),
            channel_id: p.channel_id.to_string(),
            enabled: p.enabled,
        })
        .collect();

    Ok(Json(body))
}

pub async fn upsert_my_preference(
    State(state): State<SharedState>,
    user: AuthUser,
    Json(body): Json<UpsertPreferenceRequest>,
) -> Result<Json<PreferenceResponse>, AppError> {
    let event_id = Uuid::parse_str(&body.event_id)
        .map_err(|_| AppError::BadRequest("invalid event_id uuid".into()))?;
    let channel_id = Uuid::parse_str(&body.channel_id)
        .map_err(|_| AppError::BadRequest("invalid channel_id uuid".into()))?;

    let pref = UserNotificationPreference {
        user_id: user.user_id,
        event_id: event_id.into(),
        channel_id: channel_id.into(),
        enabled: body.enabled,
    };

    state
        .notification_repo
        .upsert_user_preference(pref.clone())
        .map_err(|_| AppError::Internal)?;

    Ok(Json(PreferenceResponse {
        user_id: pref.user_id.to_string(),
        event_id: pref.event_id.to_string(),
        channel_id: pref.channel_id.to_string(),
        enabled: pref.enabled,
    }))
}

// =========================
// Group-scoped preferences (intended to be mounted under /group/{id}/... with guard)
// =========================

pub async fn get_group_preferences(
    State(state): State<SharedState>,
    Path(group_id): Path<Uuid>,
    user: AuthUser,
) -> Result<Json<Vec<GroupPreferenceResponse>>, AppError> {
    let prefs = state
        .notification_repo
        .get_group_preferences(user.user_id, GroupId(group_id))
        .map_err(|_| AppError::Internal)?;

    let body = prefs
        .into_iter()
        .map(|p| GroupPreferenceResponse {
            user_id: p.user_id.to_string(),
            group_id: p.group_id.to_string(),
            event_id: p.event_id.to_string(),
            channel_id: p.channel_id.to_string(),
            enabled: p.enabled,
        })
        .collect();

    Ok(Json(body))
}

pub async fn upsert_group_preference(
    State(state): State<SharedState>,
    Path(group_id): Path<Uuid>,
    user: AuthUser,
    Json(body): Json<UpsertPreferenceRequest>,
) -> Result<Json<GroupPreferenceResponse>, AppError> {
    let event_id = Uuid::parse_str(&body.event_id)
        .map_err(|_| AppError::BadRequest("invalid event_id uuid".into()))?;
    let channel_id = Uuid::parse_str(&body.channel_id)
        .map_err(|_| AppError::BadRequest("invalid channel_id uuid".into()))?;

    let pref = GroupNotificationPreference {
        user_id: user.user_id,
        group_id: GroupId(group_id),
        event_id: event_id.into(),
        channel_id: channel_id.into(),
        enabled: body.enabled,
    };

    state
        .notification_repo
        .upsert_group_preference(pref.clone())
        .map_err(|_| AppError::Internal)?;

    Ok(Json(GroupPreferenceResponse {
        user_id: pref.user_id.to_string(),
        group_id: pref.group_id.to_string(),
        event_id: pref.event_id.to_string(),
        channel_id: pref.channel_id.to_string(),
        enabled: pref.enabled,
    }))
}
