use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug)]
pub struct Ticket {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub status: String, // Open, In Progress, Closed
    pub created_at: NaiveDateTime,
}

#[derive(Deserialize)]
pub struct CreateTicket {
    pub title: String,
    pub description: String,
}

#[derive(Deserialize)]
pub struct UpdateTicket {
    pub title: Option<String>,
    pub description: Option<String>,
    pub status: Option<String>,
}