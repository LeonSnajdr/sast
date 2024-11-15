use chrono::Utc;

use crate::contracts::setting_contracts::{InitSettingContract, SettingContract};
use crate::prelude::*;
use crate::repositories::setting_repository;

pub async fn init_setting(init_contract: &InitSettingContract) -> Result<SettingContract> {
	let meta_date_updated = Utc::now();

	let setting_model = setting_repository::init_setting(
		&meta_date_updated,
		&init_contract.presentation_language,
		&init_contract.presentation_theme,
	)
	.await?;

	let setting_contract = SettingContract::from(setting_model);

	Ok(setting_contract)
}

pub async fn get_is_setting_empty() -> Result<bool> {
	let is_setting_empty = setting_repository::get_is_setting_empty().await?;

	Ok(is_setting_empty)
}

pub async fn get_setting() -> Result<SettingContract> {
	let setting_model = setting_repository::get_setting().await?;

	let setting_contract = SettingContract::from(setting_model);

	Ok(setting_contract)
}
