use crate::dtos::project_dtos::ProjectDto;
use crate::{db, prelude::*};

pub async fn create_project() -> Result<()> {
	sqlx::query!("insert into project (name) values ($1)", "leon")
		.execute(db::get_pool()?)
		.await?;

	Ok(())
}

pub async fn get_all_projects() -> Result<Vec<ProjectDto>> {
	let projects = sqlx::query_as!(ProjectDto, "select * from project")
		.fetch_all(db::get_pool()?)
		.await?;

	Ok(projects)
}
