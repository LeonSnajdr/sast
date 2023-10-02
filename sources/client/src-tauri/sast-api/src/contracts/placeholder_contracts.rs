use serde::Deserialize;
use specta::Type;

#[derive(Deserialize, Type)]
pub struct CreatePlaceholderContract {
    pub name: String,
    pub value: String,
    pub project_id: String,
}

#[derive(Deserialize, Type)]
pub struct UpdatePlaceholderContract {
    pub id: String,
    pub value: String,
}
