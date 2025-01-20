use crate::prelude::*;
use crate::setting::setting_contracts::{SettingInitializeContract, SettingContract, SettingUpdateContract};
use crate::setting::setting_service;

#[tauri::command]
#[specta::specta]
pub async fn setting_initialize(initialize_contract: SettingInitializeContract) -> Result<SettingContract> {
	let setting = setting_service::setting_initialize(&initialize_contract).await?;

	Ok(setting)
}

#[tauri::command]
#[specta::specta]
pub async fn setting_get_default() -> Result<Option<SettingContract>> {
	let setting = setting_service::setting_get_default().await?;

	Ok(setting)
}

#[tauri::command]
#[specta::specta]
pub async fn setting_update_one(update_contract: SettingUpdateContract) -> Result<()> {
	setting_service::setting_update_one(&update_contract).await?;

	Ok(())
}
