use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Note {
    pub id: i64,
    pub title: String,
    pub content: String,
}

#[derive(Serialize, Deserialize)]
pub struct NewNote {
    pub title: String,
    pub content: String,
}
