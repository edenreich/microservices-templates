use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Payment {
    pub id: u32,
    pub amount: u32,
    pub currency: String,
    pub status: String,
}
