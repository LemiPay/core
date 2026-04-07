use crate::data::state::SharedState;
use crate::handlers::group::{
    create_group, delete_group, get_group_by_id, get_group_members, get_user_groups,
    make_group_admin, update_group,
};

use crate::security::middlewares::auth::auth_middleware;
use crate::security::middlewares::is_in_group::{
    is_group_admin_middleware, is_in_group_middleware,
};

use axum::{
    Router, middleware,
    routing::{delete, get, post, put},
};

pub fn group_routes(state: SharedState) -> Router {
    Router::new()
        .route("/create", post(create_group))
        .route(
            "/{id}",
            get(get_group_by_id).route_layer(middleware::from_fn_with_state(
                state.clone(),
                is_in_group_middleware,
            )),
        )
        .route(
            "/{id}",
            delete(delete_group)
                .put(update_group)
                .route_layer(middleware::from_fn_with_state(
                    state.clone(),
                    is_group_admin_middleware,
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
        .route("/my-groups", get(get_user_groups))
        .route_layer(middleware::from_fn(auth_middleware))
        .with_state(state)
}
