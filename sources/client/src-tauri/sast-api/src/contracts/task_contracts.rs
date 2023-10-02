use serde::Deserialize;
use specta::Type;

#[derive(Deserialize, Type)]
pub struct CreateTaskContract {
    pub variety: String,
    pub command: String,
    pub working_directory: String,
    pub delay: i32,
    pub task_set_id: String,
}
