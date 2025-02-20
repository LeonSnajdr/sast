use chrono::Utc;

use crate::prelude::*;
use crate::setting::setting_contracts::{SettingContract, SettingInitializeContract, SettingUpdateContract};
use crate::setting::setting_repository;

pub async fn initialize(initialize_contract: &SettingInitializeContract) -> Result<SettingContract> {
	let already_initialized = setting_repository::get_is_initialized().await?;

	if already_initialized {
		return Err(Error::AlreadyExists);
	}

	let meta_date_updated = Utc::now();

	let setting_model = setting_repository::initialize(
		&meta_date_updated,
		&initialize_contract.presentation_language,
		&initialize_contract.presentation_theme,
	)
	.await?;

	let setting_contract = SettingContract::from(setting_model);

	Ok(setting_contract)
}

pub async fn get_default() -> Result<Option<SettingContract>> {
	let setting_model_option = setting_repository::get_default().await?;

	match setting_model_option {
		Some(setting_model) => Ok(Some(SettingContract::from(setting_model))),
		None => Ok(None),
	}
}

pub async fn update_one(update_contract: &SettingUpdateContract) -> Result<()> {
	setting_repository::update_one(&update_contract.id, &update_contract.presentation_language, &update_contract.presentation_theme).await?;

	Ok(())
}
