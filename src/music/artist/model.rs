use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Artist {
    pub id: String,
    pub name: String,
    pub image_url: String,
    pub country_code: Option<String>,
    pub description: Option<String>,
    pub collective_members: Option<Vec<CollectiveMember>>,
    pub logo_url: Option<String>,
    pub urls: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectiveMember {
    pub id: String,
    pub joined: Option<NaiveDate>,
    pub left: Option<NaiveDate>,
}
