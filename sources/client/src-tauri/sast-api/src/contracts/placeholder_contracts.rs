use serde::Deserialize;
use specta::Type;

#[derive(Deserialize, Type)]
pub struct CreatePlaceholderContract {
    pub name: String,
    pub variety: String,
    pub project_id: String,
}
