use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::db;
use crate::models::project_models::ProjectModel;
use crate::prelude::*;

pub async fn create_project(name: &String, date_created: &DateTime<Utc>, date_last_opened: &DateTime<Utc>) -> Result<ProjectModel> {
	let id = uuid::Uuid::new_v4();

	let project = sqlx::query_as!(
		ProjectModel,
		r#"--sql
            insert into project 
                (id, name, date_created, date_last_opened) 
                values 
                ($1, $2, $3, $4) 
            returning
                id as "id: Uuid",
                name, 
                date_created as "date_created: DateTime<Utc>",
                date_last_opened as "date_last_opened: DateTime<Utc>"
        "#,
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
		r#"--sql
            select 
                id as "id: Uuid",
                name,
                date_created as "date_created: DateTime<Utc>",
                date_last_opened as "date_last_opened: DateTime<Utc>"
            from project
            order by date_last_opened desc
        "#
	)
	.fetch_all(db::get_pool())
	.await
	.map_err(|_| Error::Db)?;

	Ok(projects)
}

pub async fn get_project(id: &Uuid) -> Result<ProjectModel> {
	let project = sqlx::query_as!(
		ProjectModel,
		r#"--sql
            select
                id as "id: Uuid",
                name,
                date_created as "date_created: DateTime<Utc>",
                date_last_opened as "date_last_opened: DateTime<Utc>"
            from project
            where id = $1
        "#,
		id
	)
	.fetch_one(db::get_pool())
	.await
	.map_err(|_| Error::Db)?;

	Ok(project)
}

pub async fn get_last_opened_project_id() -> Result<Option<Uuid>> {
	let last_opened_project_id = sqlx::query_scalar!(
		r#"--sql
        select id as "id: Uuid"
        from project
        order by date_last_opened desc
        limit 1
    "#
	)
	.fetch_optional(db::get_pool())
	.await
	.map_err(|_| Error::Db)?;

	Ok(last_opened_project_id)
}

pub async fn update_project_last_opened(id: &Uuid, date_last_opened: &DateTime<Utc>) -> Result<()> {
	sqlx::query!(
		r#"--sql 
            update project set date_last_opened = $1 where id = $2
        "#,
		date_last_opened,
		id,
	)
	.execute(db::get_pool())
	.await
	.map_err(|_| Error::Db)?;

	Ok(())
}
