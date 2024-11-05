use crate::db;
use crate::dtos::project_dtos::{CreateProject, Project};
use crate::prelude::*;

pub async fn create_project(create_dto: &CreateProject) -> Result<()> {
	let id = uuid::Uuid::new_v4();

	sqlx::query!(
		"insert into project (id, name) values ($1, $2)",
		id,
		create_dto.name
	)
	.execute(db::get_pool())
	.await
	.map_err(|_| Error::Db)?;

	Ok(())
}

pub async fn get_all_projects() -> Result<Vec<Project>> {
	let projects = sqlx::query_as!(
		Project,
		r#"select id as "id: uuid::Uuid", name from project"#
	)
	.fetch_all(db::get_pool())
	.await
	.map_err(|_| Error::Db)?;

	Ok(projects)
}
