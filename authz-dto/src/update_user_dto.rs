use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateUserDto {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
}