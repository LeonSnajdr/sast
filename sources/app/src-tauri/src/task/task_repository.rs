use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::db;
use crate::prelude::*;
use crate::task::task_models::{TaskInfoModel, TaskModel, TaskUpdateModel};

pub async fn task_create(create_model: TaskModel) -> Result<TaskModel> {
	let task = sqlx::query_as!(
		TaskModel,
		r#"--sql
            insert into task
                (id, project_id, name, tab_name, blocking, no_exit, date_created, date_last_updated)
                values
                ($1, $2, $3, $4, $5, $6, $7, $8)
			returning
                id as "id: Uuid",
                project_id as "project_id: Uuid",
                name,
                tab_name,
                blocking,
                no_exit,
                date_created as "date_created: DateTime<Utc>",
                date_last_updated as "date_last_updated: DateTime<Utc>"
        "#,
		create_model.id,
		create_model.project_id,
		create_model.name,
		create_model.tab_name,
		create_model.blocking,
		create_model.no_exit,
		create_model.date_created,
		create_model.date_last_updated
	)
	.fetch_one(db::get_pool())
	.await
	.map_err(|err| Error::Db(err.to_string()))?;

	Ok(task)
}

pub async fn task_get_info_all_project(project_id: Uuid) -> Result<Vec<TaskInfoModel>> {
	let tasks = sqlx::query_as!(
		TaskInfoModel,
		r#"--sql
            select
                id as "id: Uuid",
                project_id as "project_id: Uuid",
                name,
                blocking,
                date_created as "date_created: DateTime<Utc>",
                date_last_updated as "date_last_updated: DateTime<Utc>"
            from task
            where project_id is $1
            order by name desc
        "#,
		project_id
	)
	.fetch_all(db::get_pool())
	.await
	.map_err(|err| Error::Db(err.to_string()))?;

	Ok(tasks)
}

pub async fn task_get_one(id: Uuid) -> Result<TaskModel> {
	let task = sqlx::query_as!(
		TaskModel,
		r#"--sql
            select
                id as "id: Uuid",
                project_id as "project_id: Uuid",
                name,
                tab_name,
                blocking,
                no_exit,
                date_created as "date_created: DateTime<Utc>",
                date_last_updated as "date_last_updated: DateTime<Utc>"
            from task
            where id = $1
        "#,
		id
	)
	.fetch_one(db::get_pool())
	.await
	.map_err(|err| Error::Db(err.to_string()))?;

	Ok(task)
}

pub async fn task_update_one(update_container: TaskUpdateModel) -> Result<()> {
	sqlx::query!(
		r#"--sql
            update task
            set
                name = $2,
                tab_name = $3,
                blocking = $4,
                no_exit = $5,
                date_last_updated = $6
            where id = $1
        "#,
		update_container.id,
		update_container.name,
		update_container.tab_name,
		update_container.blocking,
		update_container.no_exit,
		update_container.date_last_updated,
	)
	.execute(db::get_pool())
	.await
	.map_err(|err| Error::Db(err.to_string()))?;

	Ok(())
}

pub async fn task_delete_one(id: Uuid) -> Result<()> {
	sqlx::query!(
		r#"--sql
            delete
            from task
            where id = $1
        "#,
		id
	)
	.execute(db::get_pool())
	.await
	.map_err(|err| Error::Db(err.to_string()))?;

	Ok(())
}
