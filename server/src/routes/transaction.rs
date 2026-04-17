use axum::routing::{get, post};
use axum::{Router, middleware};

use crate::data::state::SharedState;
use crate::handlers::transaction::{
    create_withdraw_proposal, execute_withdraw_proposal, fund_group, get_transaction,
    list_transactions,
};

use crate::security::middlewares::auth::auth_middleware;
use crate::security::middlewares::is_in_group::is_in_group_middleware;

pub fn transaction_routes(state: SharedState) -> Router {
    Router::new()
        // Create: fondear grupo (GroupWallet + UserWallet)
        .route("/{group_id}/fund", post(fund_group))
        // Create: propuesta de retiro (queda aprobada automáticamente)
        .route(
            "/{group_id}/withdraw/proposal",
            post(create_withdraw_proposal),
        )
        // Create: ejecutar retiro aprobado (sale la plata)
        .route(
            "/{group_id}/withdraw/execute",
            post(execute_withdraw_proposal),
        )
        // Read
        .route("/{group_id}/list", get(list_transactions))
        .route("/{group_id}/one", get(get_transaction))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            is_in_group_middleware,
        ))
        .route_layer(middleware::from_fn(auth_middleware))
        .with_state(state)
}
