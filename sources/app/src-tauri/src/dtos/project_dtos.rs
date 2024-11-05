use serde::{Deserialize, Serialize};
use specta::Type;
use sqlx::FromRow;

#[derive(Debug, Type, Serialize)]
pub enum ProjectError {
	CreationFailed,
}

#[derive(Debug, Type, Serialize, Deserialize)]
pub struct CreateProject {
	pub name: String,
}

#[derive(Debug, FromRow, Type, Serialize, Deserialize)]
pub struct Project {
	pub id: uuid::Uuid,
	pub name: String,
}
