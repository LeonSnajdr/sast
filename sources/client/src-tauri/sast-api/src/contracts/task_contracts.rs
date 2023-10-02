use serde::Deserialize;
use specta::Type;

#[derive(Deserialize, Type)]
pub struct CreateTaskContract {
    pub command: String,
    pub working_directory: String,
    pub delay: i32,
    pub task_set_id: String,
}

#[derive(Deserialize, Type)]
pub struct UpdateTaskContract {
    pub id: String,
    pub command: String,
    pub working_directory: String,
    pub delay: i32,
}
