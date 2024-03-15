use sqlx::MySqlPool;

pub struct AppConfig {
    pub name: &'static str,
    pub pool: MySqlPool,
}

impl AppConfig {
    pub fn new(name: &'static str, pool: MySqlPool) -> Self {
        Self { name, pool }
    }
}