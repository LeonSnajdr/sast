use prisma_client_rust::Direction;
use serde::Deserialize;
use specta::Type;

use crate::prisma::{project, task_set};

project::include!(full_project_contract {
    placeholders
    task_sets(vec![]).order_by(task_set::order::order(Direction::Asc)): include { tasks }
});

project::select!(list_project_contract { id name });

#[derive(Deserialize, Type)]
pub struct CreateProjectContract {
    pub name: String,
}

#[derive(Deserialize, Type)]
pub struct UpdateProjectContract {
    pub id: String,
    pub name: String,
}
