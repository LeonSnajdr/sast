#[derive(Debug, sqlx::FromRow, serde::Serialize, serde::Deserialize)]
pub struct ProjectDto {
	pub id: i64,
	pub name: String,
}
