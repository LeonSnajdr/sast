use serde::Deserialize;
use specta::Type;

use crate::prisma::task_set;

task_set::include!(full_task_set_contract { tasks });

task_set::include!(project_task_set_contract { project: include { placeholders } tasks });

#[derive(Deserialize, Type)]
pub struct CreateTaskSetContract {
    pub project_id: String,
    pub name: String,
}
