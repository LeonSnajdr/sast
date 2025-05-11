use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::db;
use crate::prelude::*;
use crate::project::project_models::{ProjectModel, ProjectUpdateModel};

pub async fn create(name: &String, date_created: &DateTime<Utc>, date_last_opened: &DateTime<Utc>) -> Result<ProjectModel> {
	let id = Uuid::new_v4();

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
                favorite,
                quick_switch_keybind,
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
	.map_err(|err| Error::Db(err.to_string()))?;

	Ok(project)
}

pub async fn get_all() -> Result<Vec<ProjectModel>> {
	let projects = sqlx::query_as!(
		ProjectModel,
		r#"--sql
            select
                id as "id: Uuid",
                name,
                favorite,
                quick_switch_keybind,
                date_created as "date_created: DateTime<Utc>",
                date_last_opened as "date_last_opened: DateTime<Utc>"
            from project
            order by date_last_opened desc
        "#
	)
	.fetch_all(db::get_pool())
	.await
	.map_err(|err| Error::Db(err.to_string()))?;

	Ok(projects)
}

pub async fn get_one(id: &Uuid) -> Result<ProjectModel> {
	let project = sqlx::query_as!(
		ProjectModel,
		r#"--sql
            select
                id as "id: Uuid",
                name,
                favorite,
                quick_switch_keybind,
                date_created as "date_created: DateTime<Utc>",
                date_last_opened as "date_last_opened: DateTime<Utc>"
            from project
            where id = $1
        "#,
		id
	)
	.fetch_one(db::get_pool())
	.await
	.map_err(|err| Error::Db(err.to_string()))?;

	Ok(project)
}

pub async fn get_last_opened() -> Result<Option<ProjectModel>> {
	let last_opened_project = sqlx::query_as!(
		ProjectModel,
		r#"--sql
        select
                id as "id: Uuid",
                name,
                favorite,
                quick_switch_keybind,
                date_created as "date_created: DateTime<Utc>",
                date_last_opened as "date_last_opened: DateTime<Utc>"
        from project
        order by date_last_opened desc
        limit 1
    "#
	)
	.fetch_optional(db::get_pool())
	.await
	.map_err(|err| Error::Db(err.to_string()))?;

	Ok(last_opened_project)
}

pub async fn update_one(update_model: ProjectUpdateModel) -> Result<()> {
	sqlx::query!(
		r#"--sql
            update project
            set
                name = $2,
                favorite = $3,
                quick_switch_keybind = $4
            where id = $1
        "#,
		update_model.id,
		update_model.name,
		update_model.favorite,
		update_model.quick_switch_keybind
	)
	.execute(db::get_pool())
	.await
	.map_err(|err| Error::Db(err.to_string()))?;

	Ok(())
}

pub async fn update_one_date_last_opened(id: &Uuid, date_last_opened: &DateTime<Utc>) -> Result<()> {
	sqlx::query!(
		r#"--sql
            update project
            set date_last_opened = $2
            where id = $1
        "#,
		id,
		date_last_opened,
	)
	.execute(db::get_pool())
	.await
	.map_err(|err| Error::Db(err.to_string()))?;

	Ok(())
}

pub async fn delete_one(id: Uuid) -> Result<()> {
	sqlx::query!(
		r#"--sql
			delete
			from
			project
			where id = $1
		"#,
		id
	)
	.execute(db::get_pool())
	.await
	.map_err(|err| Error::Db(err.to_string()))?;

	Ok(())
}
