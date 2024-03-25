use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use sqlx::types::chrono;

/// This struct represents a permission.
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Permission {
    pub id: i32,
    pub resource_id: i32,
    pub name: String,
    pub description: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub deleted_at: Option<chrono::DateTime<chrono::Utc>>,
}