use serde::{Deserialize, Serialize};

// #[derive(Serialize, Queryable, Debug)]
#[derive(Queryable, Debug)]
pub struct Entity {
    pub id: i32,
    pub created_timestamp: Option<chrono::NaiveDateTime>,
    pub context: Option<String>,
    pub key: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiEntity {
    pub context: Option<String>,
    pub key: Option<String>,
    pub value: Option<String>,
}
