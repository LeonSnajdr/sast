use specta::{Type};
use serde::Deserialize;

#[derive(Deserialize, Type)]
pub struct CreateProjectContract {
    pub name: String,
}