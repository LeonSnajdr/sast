use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::db;
use crate::placeholder::placeholder_models::PlaceholderModel;
use crate::prelude::*;

pub async fn create_placeholder(project_id: &Option<Uuid>, name: &String, value: &String, date_created: &DateTime<Utc>, date_last_updated: &DateTime<Utc>) -> Result<PlaceholderModel> {
    let id = Uuid::new_v4();

    let placeholder = sqlx::query_as!(
		PlaceholderModel,
		r#"--sql
            insert into placeholder
                (id, project_id, name, value, date_created, date_last_updated)
                values
                ($1, $2, $3, $4, $5, $6)
            returning
                id as "id: Uuid",
                project_id as "project_id: Uuid",
                name,
                value,
                date_created as "date_created: DateTime<Utc>",
                date_last_updated as "date_last_updated: DateTime<Utc>"
        "#,
		id,
		project_id,
		name,
		value,
		date_created,
		date_last_updated
	)
        .fetch_one(db::get_pool())
        .await
        .map_err(|_| Error::Db)?;

    Ok(placeholder)
}

pub async fn get_placeholders(project_id: &Option<Uuid>) -> Result<Vec<PlaceholderModel>> {
    let placeholders = sqlx::query_as!(
		PlaceholderModel,
		r#"--sql
            select
                id as "id: Uuid",
                project_id as "project_id: Uuid",
                name,
                value,
                date_created as "date_created: DateTime<Utc>",
                date_last_updated as "date_last_updated: DateTime<Utc>"
            from placeholder
            where project_id is not distinct from $1
            order by name desc
        "#,
		project_id
	)
        .fetch_all(db::get_pool())
        .await
        .map_err(|_| Error::Db)?;

    Ok(placeholders)
}

pub async fn get_placeholder(id: &Uuid) -> Result<PlaceholderModel> {
    let placeholder = sqlx::query_as!(
		PlaceholderModel,
		r#"--sql
            select
                id as "id: Uuid",
                project_id as "project_id: Uuid",
                name,
                value,
                date_created as "date_created: DateTime<Utc>",
                date_last_updated as "date_last_updated: DateTime<Utc>"
            from placeholder
            where id = $1
        "#,
		id
	)
        .fetch_one(db::get_pool())
        .await
        .map_err(|_| Error::Db)?;

    Ok(placeholder)
}
