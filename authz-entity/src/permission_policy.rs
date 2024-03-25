use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use sqlx::types::chrono;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct PermissionPolicy {
    pub id: i32,
    pub permission_id: i32,
    pub policy_id: i32,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub deleted_at: Option<chrono::DateTime<chrono::Utc>>,
}