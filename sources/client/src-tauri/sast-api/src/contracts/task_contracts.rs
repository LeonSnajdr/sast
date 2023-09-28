use serde::Deserialize;
use specta::Type;

#[derive(Deserialize, Type)]
pub struct CreateTaskContract {
    pub variety: String,
    pub command: String,
    pub task_set_id: String,
}
