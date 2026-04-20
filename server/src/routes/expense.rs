use axum::routing::{delete, get, post, put};
use axum::{Router, middleware};

use crate::data::state::SharedState;
use crate::handlers::expense::{
    admin_delete_expense, admin_update_expense, create_expense, delete_expense, get_expenses,
    update_expense,
};

use crate::security::middlewares::auth::auth_middleware;
use crate::security::middlewares::is_in_group::{
    is_group_admin_middleware, is_in_group_middleware,
};

pub fn expense_routes(state: SharedState) -> Router {
    Router::new()
        // Create: cualquiera del grupo puede cargar una expense
        .route(
            "/new/{group_id}",
            post(create_expense).route_layer(middleware::from_fn_with_state(
                state.clone(),
                is_in_group_middleware,
            )),
        )
        // Read: cualquiera del grupo puede ver las expenses del grupo
        .route(
            "/{group_id}/list",
            get(get_expenses).route_layer(middleware::from_fn_with_state(
                state.clone(),
                is_in_group_middleware,
            )),
        )
        // Update (admin): modificar cualquier expense del grupo
        .route(
            "/admin/{group_id}/{expense_id}",
            put(admin_update_expense).route_layer(middleware::from_fn_with_state(
                state.clone(),
                is_group_admin_middleware,
            )),
        )
        // Delete (admin): borrado lógico de cualquier expense del grupo
        .route(
            "/admin/{group_id}/{expense_id}",
            delete(admin_delete_expense).route_layer(middleware::from_fn_with_state(
                state.clone(),
                is_group_admin_middleware,
            )),
        )
        // Update (owner): el creador modifica los datos cargados
        // Delete (owner): borrado lógico de la expense por su creador
        .route("/{expense_id}", put(update_expense).delete(delete_expense))
        .route_layer(middleware::from_fn(auth_middleware))
        .with_state(state)
}
