use crate::contracts::setting_contracts::{InitSettingContract, SettingContract};
use crate::prelude::*;
use crate::services::setting_service;

#[tauri::command]
#[specta::specta]
pub async fn init_setting(init_contract: InitSettingContract) -> Result<SettingContract> {
	let setting = setting_service::init_setting(&init_contract).await?;

	Ok(setting)
}

#[tauri::command]
#[specta::specta]
pub async fn get_is_setting_empty() -> Result<bool> {
	let is_setting_empty = setting_service::get_is_setting_empty().await?;

	Ok(is_setting_empty)
}

#[tauri::command]
#[specta::specta]
pub async fn get_setting() -> Result<SettingContract> {
	let setting = setting_service::get_setting().await?;

	Ok(setting)
}
