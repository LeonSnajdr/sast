use sqlx::FromRow;

#[derive(Debug, FromRow)]
pub struct ProjectModel {
	pub id: i64,
	pub name: String,
}
