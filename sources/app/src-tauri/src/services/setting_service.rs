use chrono::Utc;

use crate::contracts::setting_contracts::{
	InitializeSettingContract, SettingContract, UpdateSettingContract,
};
use crate::prelude::*;
use crate::repositories::setting_repository;

pub async fn initialize_setting(
	init_contract: &InitializeSettingContract,
) -> Result<SettingContract> {
	let already_initializied = setting_repository::get_is_setting_initialized().await?;

	if already_initializied {
		return Err(Error::AlreadyExists);
	}

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

pub async fn get_setting() -> Result<Option<SettingContract>> {
	let setting_model_option = setting_repository::get_setting().await?;

	match setting_model_option {
		Some(setting_model) => Ok(Some(SettingContract::from(setting_model))),
		None => Ok(None),
	}
}

pub async fn update_setting(update_contract: &UpdateSettingContract) -> Result<()> {
	setting_repository::update_setting(
		&update_contract.id,
		&update_contract.presentation_language,
		&update_contract.presentation_theme,
	)
	.await?;

	Ok(())
}
