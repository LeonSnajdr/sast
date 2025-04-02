use crate::prelude::*;
use crate::setting::setting_contracts::{SettingContract, SettingInitializeContract, SettingUpdateContract};
use crate::setting::setting_models::{SettingModel, SettingUpdateModel};
use crate::setting::setting_repository;
use chrono::Utc;
use uuid::Uuid;

pub async fn initialize(initialize_contract: SettingInitializeContract) -> Result<SettingContract> {
	let already_initialized = setting_repository::get_is_initialized().await?;

	if already_initialized {
		return Err(Error::AlreadyExists);
	}

	let id = Uuid::new_v4();
	let meta_date_updated = Utc::now();
	let behavior_open_welcome = true;

	let create_model = SettingModel::from_initialize_contract(id, meta_date_updated, behavior_open_welcome, initialize_contract);

	let setting_model = setting_repository::create_one(create_model).await?;

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

pub async fn update_one(update_contract: SettingUpdateContract) -> Result<()> {
	let meta_date_updated = Utc::now();
	let update_model = SettingUpdateModel::from(meta_date_updated, update_contract);

	setting_repository::update_one(update_model).await?;

	Ok(())
}
