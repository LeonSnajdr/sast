use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::db;
use crate::prelude::*;
use crate::task_set::task::task_set_task_models::{TaskSetTaskInfoModel, TaskSetTaskModel};

pub async fn create_or_replace(task_set_id: Uuid, create_models: Vec<TaskSetTaskModel>) -> Result<()> {
	let mut tx = db::get_pool().begin().await.map_err(|err| Error::Db(err.to_string()))?;

	sqlx::query!(
		r#"--sql
			delete
			from task_set_task
			where
				task_set_id is $1
		"#,
		task_set_id
	)
	.execute(&mut *tx)
	.await
	.map_err(|err| Error::Db(err.to_string()))?;

	for create_model in create_models.iter() {
		sqlx::query!(
			r#"--sql
				insert into task_set_task
					(id, task_id, task_set_id, position, blocking, jump_into)
				values
					($1, $2, $3, $4, $5, $6)
			"#,
			create_model.id,
			create_model.task_id,
			create_model.task_set_id,
			create_model.position,
			create_model.blocking,
			create_model.jump_into,
		)
		.execute(&mut *tx)
		.await
		.map_err(|err| Error::Db(err.to_string()))?;
	}

	tx.commit().await.map_err(|err| Error::Db(err.to_string()))?;

	Ok(())
}

pub async fn get_all(task_set_id: Uuid) -> Result<Vec<TaskSetTaskModel>> {
	let task_set_tasks = sqlx::query_as!(
		TaskSetTaskModel,
		r#"--sql
            select
                id as "id: Uuid",
                task_id as "task_id: Uuid",
            	task_set_id as "task_set_id: Uuid",
            	blocking,
            	position,
                jump_into
            from task_set_task
            where task_set_id is $1
			order by position asc
        "#,
		task_set_id
	)
	.fetch_all(db::get_pool())
	.await
	.map_err(|err| Error::Db(err.to_string()))?;

	Ok(task_set_tasks)
}

pub async fn get_all_info(task_set_id: Uuid) -> Result<Vec<TaskSetTaskInfoModel>> {
	let task_set_tasks = sqlx::query_as!(
		TaskSetTaskInfoModel,
		r#"--sql
            select
                tst.task_id as "task_id: Uuid",
                t.name as task_name,
                t.date_created as "task_date_created: DateTime<Utc>",
                t.date_last_updated as "task_date_last_updated: DateTime<Utc>",
                tst.task_set_id as "task_set_id: Uuid",
            	tst.blocking,
                tst.jump_into
            from task_set_task as tst
            join task as t on t.id = tst.task_id
            where tst.task_set_id is $1
			order by position asc
        "#,
		task_set_id
	)
	.fetch_all(db::get_pool())
	.await
	.map_err(|err| Error::Db(err.to_string()))?;

	Ok(task_set_tasks)
}

pub async fn get_all_task_ids(task_set_id: Uuid) -> Result<Vec<Uuid>> {
	let task_ids = sqlx::query_scalar!(
		r#"--sql
            select
            	task_id as "task_id: Uuid"
            from task_set_task
            where task_set_id is $1
        "#,
		task_set_id
	)
	.fetch_all(db::get_pool())
	.await
	.map_err(|err| Error::Db(err.to_string()))?;

	Ok(task_ids)
}
