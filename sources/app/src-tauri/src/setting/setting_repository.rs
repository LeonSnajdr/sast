use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::prelude::*;
use crate::db;
use crate::setting::setting_models::SettingModel;

pub async fn setting_initialize(meta_date_updated: &DateTime<Utc>, presentation_language: &String, presentation_theme: &String) -> Result<SettingModel> {
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

pub async fn setting_get_is_initialized() -> Result<bool> {
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

pub async fn setting_get_default() -> Result<Option<SettingModel>> {
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

pub async fn setting_update_one(id: &Uuid, presentation_language: &String, presentation_theme: &String) -> Result<()> {
	let date_updated = Utc::now();

	sqlx::query!(
		r#"--sql
            update setting 
            set
                meta_date_updated = $1,
                presentation_language = $2,
                presentation_theme = $3
            where id = $4
        "#,
		date_updated,
		presentation_language,
		presentation_theme,
		id
	)
	.execute(db::get_pool())
	.await
	.map_err(|_| Error::Db)?;

	Ok(())
}
