use serde::{Deserialize, Serialize};

use crate::structs::user::User;

#[derive(Debug, Deserialize, Serialize)]
pub struct Credit {
    pub user: Option<User>,
    pub role: Option<String>,
}
