use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::db;
use crate::task::task_models::{TaskInfoModel, TaskModel, TaskUpdateModel};
use crate::prelude::*;

pub async fn task_create(create_model: TaskModel) -> Result<()> {
    sqlx::query!(
		r#"--sql
            insert into task
                (id, project_id, name, blocking, date_created, date_last_updated)
                values
                ($1, $2, $3, $4, $5, $6)
        "#,
		create_model.id,
		create_model.project_id,
		create_model.name,
		create_model.blocking,
        create_model.date_created,
        create_model.date_last_updated
	)
        .execute(db::get_pool())
        .await
        .map_err(|_| Error::Db)?;

    Ok(())
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
		.map_err(|_| Error::Db)?;

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
                blocking,
                date_created as "date_created: DateTime<Utc>",
                date_last_updated as "date_last_updated: DateTime<Utc>"
            from task
            where id = $1
        "#,
		id
	)
		.fetch_one(db::get_pool())
		.await
		.map_err(|_| Error::Db)?;

	Ok(task)
}

pub async fn task_update_one(update_container: TaskUpdateModel) -> Result<()> {
	sqlx::query!(
		r#"--sql
            update task
            set
                name = $2,
                blocking = $3,
                date_last_updated = $4
            where id = $1
        "#,
		update_container.id,
		update_container.name,
		update_container.blocking,
		update_container.date_last_updated,
	)
		.execute(db::get_pool())
		.await
		.map_err(|_| Error::Db)?;

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
        .map_err(|_| Error::Db)?;

    Ok(())
}
