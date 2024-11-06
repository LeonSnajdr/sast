use crate::db;
use crate::models::project_models::ProjectModel;
use crate::prelude::*;

pub async fn create_project(name: &String) -> Result<ProjectModel> {
	let project = sqlx::query_as!(
		ProjectModel,
		"insert into project (name) values ($1) returning *",
		name
	)
	.fetch_one(db::get_pool())
	.await
	.map_err(|_| Error::Db)?;

	Ok(project)
}

pub async fn get_all_projects() -> Result<Vec<ProjectModel>> {
	let projects = sqlx::query_as!(ProjectModel, "select * from project")
		.fetch_all(db::get_pool())
		.await
		.map_err(|_| Error::Db)?;

	Ok(projects)
}

pub async fn get_project(id: &i32) -> Result<ProjectModel> {
	let project = sqlx::query_as!(ProjectModel, "select * from project where id = $1", id)
		.fetch_one(db::get_pool())
		.await
		.map_err(|_| Error::Db)?;

	Ok(project)
}
