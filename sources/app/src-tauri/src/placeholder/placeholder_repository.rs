use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::db;
use crate::placeholder::placeholder_enums::{PlaceholderKind, PlaceholderSource, PlaceholderVisibility};
use crate::placeholder::placeholder_models::{PlaceholderModel, PlaceholderUpdateModel};
use crate::prelude::*;

pub async fn create(create_model: PlaceholderModel) -> Result<PlaceholderModel> {
	let placeholder = sqlx::query_as!(
		PlaceholderModel,
		r#"--sql
            insert into placeholder
                (id, project_id, name, favorite, value, visibility, kind, source, date_created, date_last_updated)
                values
                ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            returning
                id as "id: Uuid",
                project_id as "project_id: Uuid",
                name,
                favorite,
                value,
                visibility as "visibility: PlaceholderVisibility",
                kind as "kind: PlaceholderKind",
                source as "source: PlaceholderSource",
                date_created as "date_created: DateTime<Utc>",
                date_last_updated as "date_last_updated: DateTime<Utc>"
        "#,
		create_model.id,
		create_model.project_id,
		create_model.name,
		create_model.favorite,
		create_model.value,
		create_model.visibility,
		create_model.kind,
		create_model.source,
		create_model.date_created,
		create_model.date_last_updated
	)
	.fetch_one(db::get_pool())
	.await
	.map_err(|err| Error::Db(err.to_string()))?;

	Ok(placeholder)
}

pub async fn get_many(project_id: Uuid) -> Result<Vec<PlaceholderModel>> {
	let placeholders = sqlx::query_as!(
		PlaceholderModel,
		r#"--sql
            select
                id as "id: Uuid",
                project_id as "project_id: Uuid",
                name,
                favorite,
                value,
                visibility as "visibility: PlaceholderVisibility",
                kind as "kind: PlaceholderKind",
                source as "source: PlaceholderSource",
                date_created as "date_created: DateTime<Utc>",
                date_last_updated as "date_last_updated: DateTime<Utc>"
            from placeholder
            where
            	project_id is $1 or
            	(project_id is not $1 and visibility is $2)
            order by favorite desc, name desc
        "#,
		project_id,
		PlaceholderVisibility::Global
	)
	.fetch_all(db::get_pool())
	.await
	.map_err(|err| Error::Db(err.to_string()))?;

	Ok(placeholders)
}

pub async fn get_many_used_in_task(task_id: Uuid) -> Result<Vec<PlaceholderModel>> {
	let placeholders = sqlx::query_as!(
		PlaceholderModel,
		r#"--sql
            select
                p.id as "id: Uuid",
                p.project_id as "project_id: Uuid",
                p.name,
                p.favorite,
                p.value,
                p.visibility as "visibility: PlaceholderVisibility",
                p.kind as "kind: PlaceholderKind",
                p.source as "source: PlaceholderSource",
                p.date_created as "date_created: DateTime<Utc>",
                p.date_last_updated as "date_last_updated: DateTime<Utc>"
            from placeholder p
            where p.id in (
                select pit.placeholder_id
                from placeholder_insert_tile pit
                where (pit.task_command_id = $1 or pit.task_working_dir_id = $1) and pit.placeholder_id is not null
            )
            order by p.name desc
        "#,
		task_id
	)
	.fetch_all(db::get_pool())
	.await
	.map_err(|err| Error::Db(err.to_string()))?;

	Ok(placeholders)
}

pub async fn get_one(id: Uuid) -> Result<PlaceholderModel> {
	let placeholder = sqlx::query_as!(
		PlaceholderModel,
		r#"--sql
            select
                id as "id: Uuid",
                project_id as "project_id: Uuid",
                name,
                favorite,
                value,
                visibility as "visibility: PlaceholderVisibility",
                kind as "kind: PlaceholderKind",
                source as "source: PlaceholderSource",
                date_created as "date_created: DateTime<Utc>",
                date_last_updated as "date_last_updated: DateTime<Utc>"
            from placeholder
            where id = $1
        "#,
		id
	)
	.fetch_one(db::get_pool())
	.await
	.map_err(|err| Error::Db(err.to_string()))?;

	Ok(placeholder)
}

pub async fn update_one(update_container: PlaceholderUpdateModel) -> Result<()> {
	sqlx::query!(
		r#"--sql
            update placeholder
            set
                project_id = $1,
                name = $2,
                favorite = $3,
                value = $4,
                visibility = $5,
                kind = $6,
                source = $7,
                date_last_updated = $8
            where id = $9
        "#,
		update_container.project_id,
		update_container.name,
		update_container.favorite,
		update_container.value,
		update_container.visibility,
		update_container.kind,
		update_container.source,
		update_container.date_last_updated,
		update_container.id
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
            from placeholder
            where id = $1
        "#,
		id
	)
	.execute(db::get_pool())
	.await
	.map_err(|err| Error::Db(err.to_string()))?;

	Ok(())
}
