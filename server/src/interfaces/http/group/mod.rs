use axum::{
    Router, middleware,
    routing::{get, post},
};

pub mod dto;
pub mod handlers;

use crate::interfaces::http::middlewares::{
    auth_middleware::auth_middleware,
    group_guard_middleware::{is_group_admin_middleware, is_in_group_middleware},
};

use handlers::{
    create_group::create_group, delete_group::delete_group, get_group::get_group,
    get_group_members::get_group_members, leave_group::leave_group, list_user_groups::list_user_groups,
    make_group_admin::make_group_admin, update_group::update_group,
};

use crate::setup::state::SharedState;

pub fn routes(state: SharedState) -> Router<SharedState> {
    Router::new()
        .route("/create", post(create_group))
        .route(
            "/{id}",
            get(get_group)
                .put(update_group)
                .delete(delete_group)
                .route_layer(middleware::from_fn_with_state(
                    state.clone(),
                    is_in_group_middleware,
                )),
        )
        .route(
            "/{id}/leave",
            post(leave_group).route_layer(middleware::from_fn_with_state(
                state.clone(),
                is_in_group_middleware,
            )),
        )
        .route(
            "/{id}/make_admin",
            post(make_group_admin).route_layer(middleware::from_fn_with_state(
                state.clone(),
                is_group_admin_middleware,
            )),
        )
        .route(
            "/{id}/members",
            get(get_group_members).route_layer(middleware::from_fn_with_state(
                state.clone(),
                is_in_group_middleware,
            )),
        )
        .route("/my-groups", get(list_user_groups))
        .route("/my", get(list_user_groups))
        .route("/", post(create_group))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ))
        .with_state(state)
}
