use axum::{
    Router, middleware,
    routing::{get, post, put},
};

use crate::{
    interfaces::http::{
        expense::handlers::{
            admin_delete_expense, admin_update_expense, create_expense, delete_expense,
            get_expenses, update_expense,
        },
        middlewares::{
            auth_middleware::auth_middleware,
            group_guard_middleware::{
                is_group_admin_for_resource_middleware, is_in_group_middleware,
            },
        },
    },
    setup::state::SharedState,
};

pub mod dto;
pub mod handlers;

pub fn routes(state: SharedState) -> Router<SharedState> {
    Router::new()
        .route(
            "/new/{group_id}",
            post(create_expense).route_layer(middleware::from_fn_with_state(
                state.clone(),
                is_in_group_middleware,
            )),
        )
        .route(
            "/{group_id}/list",
            get(get_expenses).route_layer(middleware::from_fn_with_state(
                state.clone(),
                is_in_group_middleware,
            )),
        )
        .route(
            "/admin/{group_id}/{expense_id}",
            put(admin_update_expense)
                .delete(admin_delete_expense)
                .route_layer(middleware::from_fn_with_state(
                    state.clone(),
                    is_group_admin_for_resource_middleware,
                )),
        )
        .route(
            "/{group_id}/{expense_id}",
            put(update_expense)
                .delete(delete_expense)
                .route_layer(middleware::from_fn_with_state(
                    state.clone(),
                    is_in_group_middleware,
                )),
        )
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ))
        .with_state(state)
}
