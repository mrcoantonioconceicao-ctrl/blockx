use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct RefreshToken {
    pub token: String,
    pub user_id: String,
    pub exp: usize,
}
