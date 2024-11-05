use crate::db;
use crate::models::project_models::ProjectModel;
use crate::prelude::*;

pub async fn create_project(name: &String) -> Result<()> {
	sqlx::query!("insert into project (name) values ($1)", name)
		.execute(db::get_pool())
		.await
		.map_err(|_| Error::Db)?;

	Ok(())
}

pub async fn get_all_projects() -> Result<Vec<ProjectModel>> {
	let projects = sqlx::query_as!(ProjectModel, r#"select * from project"#)
		.fetch_all(db::get_pool())
		.await
		.map_err(|_| Error::Db)?;

	Ok(projects)
}
