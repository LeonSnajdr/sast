use crate::prelude::*;
use crate::setting::setting_contracts::{InitializeSettingContract, SettingContract, UpdateSettingContract};
use crate::setting::setting_service;

#[tauri::command]
#[specta::specta]
pub async fn initialize_setting(init_contract: InitializeSettingContract) -> Result<SettingContract> {
	let setting = setting_service::initialize_setting(&init_contract).await?;

	Ok(setting)
}

#[tauri::command]
#[specta::specta]
pub async fn get_setting() -> Result<Option<SettingContract>> {
	let setting = setting_service::get_setting().await?;

	Ok(setting)
}

#[tauri::command]
#[specta::specta]
pub async fn update_setting(update_contract: UpdateSettingContract) -> Result<()> {
	setting_service::update_setting(&update_contract).await?;

	Ok(())
}
