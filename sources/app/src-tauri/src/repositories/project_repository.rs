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
	let projects = sqlx::query_as!(
		ProjectModel,
		"select * from project order by date_last_opened desc"
	)
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

pub async fn get_last_opened_project() -> Result<Option<ProjectModel>> {
	let last_opened_project =
		sqlx::query_as!(ProjectModel, "select id, name, date_created, date_last_opened from project order by date_last_opened desc limit 1")
			.fetch_optional(db::get_pool())
			.await
			.map_err(|_| Error::Db)?;

	Ok(last_opened_project)
}

pub async fn update_project_last_opened(
	id: &String,
	date_last_opened: &DateTime<Utc>,
) -> Result<()> {
	sqlx::query!(
		"update project set date_last_opened = $1 where id = $2",
		date_last_opened,
		id,
	)
	.execute(db::get_pool())
	.await
	.map_err(|_| Error::Db)?;

	Ok(())
}
