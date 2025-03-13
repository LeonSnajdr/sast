use crate::setting::setting_contracts::{SettingContract, SettingInitializeContract, SettingUpdateContract};
use crate::setting::setting_models::{SettingModel, SettingUpdateModel};
use chrono::{DateTime, Utc};
use uuid::Uuid;

impl SettingContract {
	pub fn from(value: SettingModel) -> Self {
		Self {
			id: value.id,
			meta_date_updated: value.meta_date_updated,
			presentation_language: value.presentation_language,
			presentation_theme: value.presentation_theme,
			behavior_open_welcome: value.behavior_open_welcome,
		}
	}
}

impl SettingModel {
	pub fn from_initialize_contract(id: Uuid, meta_date_updated: DateTime<Utc>, behavior_open_welcome: bool, value: SettingInitializeContract) -> Self {
		Self {
			id,
			meta_date_updated,
			presentation_language: value.presentation_language,
			presentation_theme: value.presentation_theme,
			behavior_open_welcome,
		}
	}
}

impl SettingUpdateModel {
	pub fn from(meta_date_updated: DateTime<Utc>, value: SettingUpdateContract) -> Self {
		Self {
			id: value.id,
			meta_date_updated,
			presentation_language: value.presentation_language,
			presentation_theme: value.presentation_theme,
			behavior_open_welcome: value.behavior_open_welcome,
		}
	}
}
