use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct TodoItem {
    pub done: bool,
    pub name: String,
    pub tags: Vec<String>,
    pub time: String,
}
