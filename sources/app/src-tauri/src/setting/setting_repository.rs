use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::db;
use crate::prelude::*;
use crate::setting::setting_models::{SettingModel, SettingUpdateModel};

pub async fn create_one(setting_model: SettingModel) -> Result<SettingModel> {
	let setting_model = sqlx::query_as!(
		SettingModel,
		r#"--sql
            insert into setting 
                (id, meta_date_updated, presentation_language, presentation_theme, behavior_open_welcome)
                values 
                ($1, $2, $3, $4, $5)
			returning
                id as "id: Uuid",
                meta_date_updated as "meta_date_updated: DateTime<Utc>",
                presentation_language,
                presentation_theme,
                behavior_open_welcome
        "#,
		setting_model.id,
		setting_model.meta_date_updated,
		setting_model.presentation_language,
		setting_model.presentation_theme,
		setting_model.behavior_open_welcome
	)
	.fetch_one(db::get_pool())
	.await
	.map_err(|err| Error::Db(err.to_string()))?;

	Ok(setting_model)
}

pub async fn get_is_initialized() -> Result<bool> {
	let setting_count = sqlx::query_scalar!(
		r#"--sql
           select count(id) from setting
        "#
	)
	.fetch_one(db::get_pool())
	.await
	.map_err(|err| Error::Db(err.to_string()))?;

	let is_setting_initialized = setting_count > 0;

	Ok(is_setting_initialized)
}

pub async fn get_default() -> Result<Option<SettingModel>> {
	let setting = sqlx::query_as!(
		SettingModel,
		r#"--sql
            select
                id as "id: Uuid",
                meta_date_updated as "meta_date_updated: DateTime<Utc>",
                presentation_language,
                presentation_theme,
                behavior_open_welcome
            from setting
            limit 1
        "#
	)
	.fetch_optional(db::get_pool())
	.await
	.map_err(|err| Error::Db(err.to_string()))?;

	Ok(setting)
}

pub async fn update_one(update_model: SettingUpdateModel) -> Result<()> {
	sqlx::query!(
		r#"--sql
            update setting
            set
                meta_date_updated = $2,
                presentation_language = $3,
                presentation_theme = $4,
                behavior_open_welcome = $5
            where id = $1
        "#,
		update_model.id,
		update_model.meta_date_updated,
		update_model.presentation_language,
		update_model.presentation_theme,
		update_model.behavior_open_welcome,
	)
	.execute(db::get_pool())
	.await
	.map_err(|err| Error::Db(err.to_string()))?;

	Ok(())
}
