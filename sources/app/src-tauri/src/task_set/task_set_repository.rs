use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::db;
use crate::prelude::*;
use crate::task_set::task_set_models::{TaskSetInfoModel, TaskSetModel, TaskSetUpdateModel};

pub async fn create(create_model: TaskSetModel) -> Result<TaskSetModel> {
	let task_set = sqlx::query_as!(
		TaskSetModel,
		r#"--sql
            insert into task_set
                (id, project_id, name, date_created, date_last_updated)
                values
                ($1, $2, $3, $4, $5)
			returning
                id as "id: Uuid",
                project_id as "project_id: Uuid",
                name,
                date_created as "date_created: DateTime<Utc>",
                date_last_updated as "date_last_updated: DateTime<Utc>"
        "#,
		create_model.id,
		create_model.project_id,
		create_model.name,
		create_model.date_created,
		create_model.date_last_updated
	)
	.fetch_one(db::get_pool())
	.await
	.map_err(|err| Error::Db(err.to_string()))?;

	Ok(task_set)
}

pub async fn get_one_info(id: Uuid) -> Result<TaskSetInfoModel> {
	let task_set_info = sqlx::query_as!(
		TaskSetInfoModel,
		r#"--sql
            select
                id as "id: Uuid",
                project_id as "project_id: Uuid",
                name,
                date_created as "date_created: DateTime<Utc>",
                date_last_updated as "date_last_updated: DateTime<Utc>"
            from task_set
            where id is $1
        "#,
		id
	)
		.fetch_one(db::get_pool())
		.await
		.map_err(|err| Error::Db(err.to_string()))?;

	Ok(task_set_info)
}

pub async fn get_many_info(project_id: Uuid) -> Result<Vec<TaskSetInfoModel>> {
	let task_sets = sqlx::query_as!(
		TaskSetInfoModel,
		r#"--sql
            select
                id as "id: Uuid",
                project_id as "project_id: Uuid",
                name,
                date_created as "date_created: DateTime<Utc>",
                date_last_updated as "date_last_updated: DateTime<Utc>"
            from task_set
            where project_id is $1
            order by name desc
        "#,
		project_id
	)
	.fetch_all(db::get_pool())
	.await
	.map_err(|err| Error::Db(err.to_string()))?;

	Ok(task_sets)
}

pub async fn get_one(id: Uuid) -> Result<TaskSetModel> {
	let task_set = sqlx::query_as!(
		TaskSetModel,
		r#"--sql
            select
                id as "id: Uuid",
                project_id as "project_id: Uuid",
                name,
                date_created as "date_created: DateTime<Utc>",
                date_last_updated as "date_last_updated: DateTime<Utc>"
            from task_set
            where id = $1
        "#,
		id
	)
	.fetch_one(db::get_pool())
	.await
	.map_err(|err| Error::Db(err.to_string()))?;

	Ok(task_set)
}

pub async fn update_one(update_container: TaskSetUpdateModel) -> Result<()> {
	sqlx::query!(
		r#"--sql
            update task_set
            set
                name = $2,
                date_last_updated = $3
            where id = $1
        "#,
		update_container.id,
		update_container.name,
		update_container.date_last_updated,
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
            from task_set
            where id = $1
        "#,
		id
	)
	.execute(db::get_pool())
	.await
	.map_err(|err| Error::Db(err.to_string()))?;

	Ok(())
}
