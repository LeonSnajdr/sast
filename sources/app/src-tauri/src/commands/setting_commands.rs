use crate::contracts::setting_contracts::{InitializeSettingContract, SettingContract};
use crate::prelude::*;
use crate::services::setting_service;

#[tauri::command]
#[specta::specta]
pub async fn initialize_setting(
	init_contract: InitializeSettingContract,
) -> Result<SettingContract> {
	let setting = setting_service::initialize_setting(&init_contract).await?;

	Ok(setting)
}

#[tauri::command]
#[specta::specta]
pub async fn get_is_setting_initialized() -> Result<bool> {
	let is_setting_initialized = setting_service::get_is_setting_initialized().await?;

	Ok(is_setting_initialized)
}

#[tauri::command]
#[specta::specta]
pub async fn get_setting() -> Result<SettingContract> {
	let setting = setting_service::get_setting().await?;

	Ok(setting)
}
