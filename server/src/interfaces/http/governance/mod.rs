use axum::{
    Router, middleware,
    routing::{delete, get, post, put},
};

use crate::interfaces::http::middlewares::{
    auth_middleware::auth_middleware,
    group_guard_middleware::{is_group_admin_middleware, is_in_group_middleware},
};
use crate::setup::state::SharedState;

pub mod dto;
pub mod handlers;

use handlers::{
    cancel_fund_round, contribute_fund_round, create_fund_round_proposal,
    create_new_member_proposal, create_withdraw_proposal, delete_proposal,
    execute_withdraw_proposal, get_all_fund_rounds, get_all_withdraw_proposals,
    get_fund_round_remaining, get_fund_round_status, get_user_contribution,
    group_new_member_proposals, my_new_member_proposals, received_new_member_proposals,
    respond_new_member_proposal,
};

pub fn routes(state: SharedState) -> Router<SharedState> {
    Router::new()
        .route("/my", get(my_new_member_proposals))
        .route("/received", get(received_new_member_proposals))
        .route(
            "/group/{group_id}",
            get(group_new_member_proposals).route_layer(middleware::from_fn_with_state(
                state.clone(),
                is_in_group_middleware,
            )),
        )
        .route(
            "/withdraw/{group_id}",
            get(get_all_withdraw_proposals).route_layer(middleware::from_fn_with_state(
                state.clone(),
                is_in_group_middleware,
            )),
        )
        .route(
            "/new-member/{group_id}",
            post(create_new_member_proposal).route_layer(middleware::from_fn_with_state(
                state.clone(),
                is_group_admin_middleware,
            )),
        )
        .route(
            "/{group_id}",
            delete(delete_proposal).route_layer(middleware::from_fn_with_state(
                state.clone(),
                is_group_admin_middleware,
            )),
        )
        .route("/respond/{proposal_id}", put(respond_new_member_proposal))
        .route(
            "/{group_id}/withdraw/proposal",
            post(create_withdraw_proposal).route_layer(middleware::from_fn_with_state(
                state.clone(),
                is_in_group_middleware,
            )),
        )
        .route(
            "/{group_id}/withdraw/execute",
            post(execute_withdraw_proposal).route_layer(middleware::from_fn_with_state(
                state.clone(),
                is_in_group_middleware,
            )),
        )
        .route(
            "/fund-round/create/{group_id}",
            post(create_fund_round_proposal).route_layer(middleware::from_fn_with_state(
                state.clone(),
                is_group_admin_middleware,
            )),
        )
        .route(
            "/fund-round/{fund_round_id}/contribute",
            post(contribute_fund_round).get(get_user_contribution),
        )
        .route(
            "/fund-round/{group_id}/get-all",
            get(get_all_fund_rounds).route_layer(middleware::from_fn_with_state(
                state.clone(),
                is_in_group_middleware,
            )),
        )
        .route("/fund-round/{fund_round_id}", get(get_fund_round_status))
        .route(
            "/fund-round/{fund_round_id}/remaining",
            get(get_fund_round_remaining),
        )
        .route(
            "/fund-round/{fund_round_id}/cancel",
            delete(cancel_fund_round),
        )
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ))
        .with_state(state)
}
