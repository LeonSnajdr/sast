use crate::contracts::project_contracts::ProjectContract;
use crate::models::project_models::ProjectModel;

impl From<ProjectModel> for ProjectContract {
	fn from(value: ProjectModel) -> Self {
		Self {
			id: value.id as i32,
			name: value.name,
			date_created: value.date_created.0,
			date_last_opened: value.date_last_opened.0,
		}
	}
}
