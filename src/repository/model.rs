use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

// the noteModel should implement the sqlx::FromRow trait so that the struct can be mapped
// to the row of the data returned by the SQL query
#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct NoteModel {
    pub id: Uuid,
    pub title: String,
    pub content: String,
    pub category: Option<String>,
    pub published: Option<bool>,
    #[serde(rename = "createdAt")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(rename= "updatedAt")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}