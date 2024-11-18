use chrono::Utc;

use crate::contracts::setting_contracts::{InitializeSettingContract, SettingContract};
use crate::prelude::*;
use crate::repositories::setting_repository;

pub async fn initialize_setting(
	init_contract: &InitializeSettingContract,
) -> Result<SettingContract> {
	let meta_date_updated = Utc::now();

	let setting_model = setting_repository::initialize_setting(
		&meta_date_updated,
		&init_contract.presentation_language,
		&init_contract.presentation_theme,
	)
	.await?;

	let setting_contract = SettingContract::from(setting_model);

	Ok(setting_contract)
}

pub async fn get_is_setting_initialized() -> Result<bool> {
	let is_setting_initialized = setting_repository::get_is_setting_initialized().await?;

	Ok(is_setting_initialized)
}

pub async fn get_setting() -> Result<SettingContract> {
	let setting_model = setting_repository::get_setting().await?;

	let setting_contract = SettingContract::from(setting_model);

	Ok(setting_contract)
}
