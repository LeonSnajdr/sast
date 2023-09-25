use serde::Deserialize;
use specta::Type;

use crate::prisma::project;

project::include!(full_project { placeholders });

#[derive(Deserialize, Type)]
pub struct CreateProjectContract {
    pub name: String,
}

#[derive(Deserialize, Type)]
pub struct UpdateProjectContract {
    pub id: String,
    pub name: String,
}
