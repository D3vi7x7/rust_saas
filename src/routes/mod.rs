use axum::{Router, routing::post};
use crate::auth::handlers::{register_user, login_user};
use crate::db::PgPool;

pub mod tickets;

pub fn create_auth_routes(pool: PgPool) -> Router {
    Router::new()
        .route("/auth/register", post(register_user))
        .route("/auth/login", post(login_user))
        .with_state(pool)
}

pub fn create_ticket_routes(pool: PgPool) -> Router {
    tickets::ticket_routes(pool)
}