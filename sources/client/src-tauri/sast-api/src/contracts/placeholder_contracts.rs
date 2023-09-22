use specta::{Type};
use serde::Deserialize;

#[derive(Deserialize, Type)]
pub struct CreatePlaceholderContract {
    pub name: String,
    pub variety: String,
    pub project_id: String
}