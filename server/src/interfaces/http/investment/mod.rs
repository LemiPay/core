use axum::{
    Router, middleware,
    routing::{get, post},
};

use crate::interfaces::http::middlewares::{
    auth_middleware::auth_middleware,
    group_guard_middleware::{is_group_admin_middleware, is_in_group_middleware},
};
use crate::setup::state::SharedState;

pub mod dto;
pub mod handlers;

use handlers::{
    create_investment_proposal, execute_investment_proposal, get_investment_snapshots,
    list_group_investments, list_strategies, withdraw_investment,
};

pub fn routes(state: SharedState) -> Router<SharedState> {
    Router::new()
        .route("/strategies", get(list_strategies))
        .route(
            "/proposal/{group_id}",
            post(create_investment_proposal).route_layer(middleware::from_fn_with_state(
                state.clone(),
                is_group_admin_middleware,
            )),
        )
        .route(
            "/execute/{group_id}",
            post(execute_investment_proposal).route_layer(middleware::from_fn_with_state(
                state.clone(),
                is_group_admin_middleware,
            )),
        )
        .route(
            "/withdraw/{group_id}",
            post(withdraw_investment).route_layer(middleware::from_fn_with_state(
                state.clone(),
                is_group_admin_middleware,
            )),
        )
        .route(
            "/{group_id}",
            get(list_group_investments).route_layer(middleware::from_fn_with_state(
                state.clone(),
                is_in_group_middleware,
            )),
        )
        .route(
            "/{investment_id}/snapshots",
            get(get_investment_snapshots).route_layer(middleware::from_fn_with_state(
                state.clone(),
                auth_middleware,
            )),
        )
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ))
        .with_state(state)
}
