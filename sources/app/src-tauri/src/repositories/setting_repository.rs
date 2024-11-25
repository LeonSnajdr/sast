use chrono::{DateTime, Utc};
use sqlx::QueryBuilder;
use uuid::Uuid;

use crate::contracts::setting_contracts::UpdateSettingContract;
use crate::db;
use crate::models::setting_models::SettingModel;
use crate::prelude::*;

pub async fn initialize_setting(
	meta_date_updated: &DateTime<Utc>,
	presentation_language: &String,
	presentation_theme: &String,
) -> Result<SettingModel> {
	let id = Uuid::new_v4();

	let setting = sqlx::query_as!(
		SettingModel,
		r#"--sql
            insert into setting 
                (id, meta_date_updated, presentation_language, presentation_theme) 
                values 
                ($1, $2, $3, $4) 
            returning
                id as "id: Uuid",
                meta_date_updated as "meta_date_updated: DateTime<Utc>",
                presentation_language,
                presentation_theme
        "#,
		id,
		meta_date_updated,
		presentation_language,
		presentation_theme
	)
	.fetch_one(db::get_pool())
	.await
	.map_err(|_| Error::Db)?;

	Ok(setting)
}

pub async fn get_is_setting_initialized() -> Result<bool> {
	let setting_count = sqlx::query_scalar!(
		r#"--sql
           select count(id) from setting
        "#
	)
	.fetch_one(db::get_pool())
	.await
	.map_err(|_| Error::Db)?;

	let is_setting_initialized = setting_count > 0;

	Ok(is_setting_initialized)
}

pub async fn get_setting() -> Result<Option<SettingModel>> {
	let setting = sqlx::query_as!(
		SettingModel,
		r#"--sql
            select 
                id as "id: Uuid",
                meta_date_updated as "meta_date_updated: DateTime<Utc>",
                presentation_language,
                presentation_theme
            from setting
            limit 1
        "#
	)
	.fetch_optional(db::get_pool())
	.await
	.map_err(|_| Error::Db)?;

	Ok(setting)
}

pub async fn update_setting(id: &Uuid, update: &UpdateSettingContract) -> Result<()> {
	let setting = sqlx::query_as!(
		SettingModel,
		r#"--sql
            update setting 
            set
                presentation_language = $1,
                presentation_theme = $2
            where id = $3
            returning
                id as "id: Uuid",
                meta_date_updated as "meta_date_updated: DateTime<Utc>",
                presentation_language,
                presentation_theme
        "#,
		update.presentation_language,
		update.presentation_theme,
		id
	)
	.fetch_one(db::get_pool())
	.await
	.map_err(|_| Error::Db)?;

	let mut query_builder: QueryBuilder<'_, _> = QueryBuilder::new(r#"--sql update setting set"#);

	let mut updates = query_builder.separated(", ");

	if let Some(presentation_language) = &update.presentation_language {
		updates
			.push_bind(presentation_language)
			.push("presentation_language = ");
	}

	if let Some(presentation_theme) = &update.presentation_theme {
		updates
			.push_bind(presentation_theme)
			.push("presentation_theme = ");
	}

	let query = query_builder.build();
	query.execute(db::get_pool()).await.map_err(|_| Error::Db)?;

	Ok(())
}
