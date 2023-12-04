use prisma_client_rust::Direction;
use serde::Deserialize;
use specta::Type;

use crate::prisma::{task, task_set};

task_set::include!(full_task_set_contract {
   tasks(vec![]).order_by(task::order::order(Direction::Asc))
});

task_set::include!(project_task_set_contract {
    project: include {
        placeholders
    }
    tasks(vec![]).order_by(task::order::order(Direction::Asc))
});

#[derive(Deserialize, Type)]
pub struct CreateTaskSetContract {
    pub project_id: String,
    pub order: i32,
    pub name: String,
    pub description: String,
}

#[derive(Deserialize, Type)]
pub struct UpdateTaskSetContract {
    pub id: String,
    pub description: String,
    pub order: i32,
}
