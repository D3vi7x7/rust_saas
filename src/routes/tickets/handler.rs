use axum::{
    Json,
    extract::{State, Path},
    http::StatusCode,
    Router, routing::{get, post, put, delete}
};
use uuid::Uuid;
use sqlx::PgPool;
use serde_json::json;

use crate::models::ticket::{Ticket, CreateTicket, UpdateTicket};

pub fn ticket_routes(pool: PgPool) -> Router {
    Router::new()
        .route("/tickets", post(create_ticket).get(get_tickets))
        .route("/tickets/:id", put(update_ticket).delete(delete_ticket))
        .with_state(pool)
}

async fn create_ticket(
    State(pool): State<PgPool>,
    Json(data): Json<CreateTicket>,
) -> Result<Json<Ticket>, (StatusCode, String)> {
    let ticket = sqlx::query_as!(
        Ticket,
        r#"
        INSERT INTO tickets (id, title, description, status)
        VALUES ($1, $2, $3, 'Open')
        RETURNING id, title, description, status, created_at
        "#,
        Uuid::new_v4(),
        data.title,
        data.description,
    )
    .fetch_one(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(ticket))
}

async fn get_tickets(State(pool): State<PgPool>) -> Result<Json<Vec<Ticket>>, (StatusCode, String)> {
    let tickets = sqlx::query_as!(
        Ticket,
        r#"SELECT id, title, description, status, created_at FROM tickets ORDER BY created_at DESC"#
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(tickets))
}

async fn update_ticket(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
    Json(update): Json<UpdateTicket>,
) -> Result<Json<Ticket>, (StatusCode, String)> {
    let ticket = sqlx::query_as!(
        Ticket,
        r#"
        UPDATE tickets
        SET title = COALESCE($2, title),
            description = COALESCE($3, description),
            status = COALESCE($4, status)
        WHERE id = $1
        RETURNING id, title, description, status, created_at
        "#,
        id,
        update.title,
        update.description,
        update.status
    )
    .fetch_one(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(ticket))
}

async fn delete_ticket(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, (StatusCode, String)> {
    sqlx::query!("DELETE FROM tickets WHERE id = $1", id)
        .execute(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(StatusCode::NO_CONTENT)
}
