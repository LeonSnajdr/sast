use crate::setting::setting_contracts::SettingContract;
use crate::setting::setting_models::SettingModel;

impl From<SettingModel> for SettingContract {
	fn from(value: SettingModel) -> Self {
		Self {
			id: value.id,
			meta_date_updated: value.meta_date_updated,
			presentation_language: value.presentation_language,
			presentation_theme: value.presentation_theme,
		}
	}
}
