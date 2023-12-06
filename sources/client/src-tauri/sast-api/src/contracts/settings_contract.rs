use serde::Deserialize;
use specta::Type;

#[derive(Deserialize, Type)]
pub struct UpdateSettingsContract {
    pub id: String,
    pub language: String,
    pub theme: String,
    pub default_dir: String,
}
