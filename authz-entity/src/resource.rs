use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use sqlx::types::chrono;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Resource {
    pub id: i32,
    pub name: String,
    pub display_name: Option<String>,
    pub res_type: Option<String>,
    pub uri: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub deleted_at: Option<chrono::DateTime<chrono::Utc>>,
}