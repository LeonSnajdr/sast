use chrono::{DateTime, Utc};

use crate::db;
use crate::models::project_models::ProjectModel;
use crate::prelude::*;

pub async fn create_project(
	name: &String,
	date_created: &DateTime<Utc>,
	date_last_opened: &DateTime<Utc>,
) -> Result<ProjectModel> {
	let id = uuid::Uuid::new_v4().to_string();

	let project = sqlx::query_as!(
		ProjectModel,
		"insert into project (id, name, date_created, date_last_opened) values ($1, $2, $3, $4) returning *",
        id,
		name,
        date_created,
        date_last_opened
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

pub async fn get_project(id: &String) -> Result<ProjectModel> {
	let project = sqlx::query_as!(
		ProjectModel,
		"select id, name, date_created, date_last_opened from project where id = $1",
		id
	)
	.fetch_one(db::get_pool())
	.await
	.map_err(|_| Error::Db)?;

	Ok(project)
}
