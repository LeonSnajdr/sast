use crate::contracts::project_contracts::ProjectContract;
use crate::models::project_models::ProjectModel;

impl From<ProjectModel> for ProjectContract {
	fn from(value: ProjectModel) -> Self {
		Self {
			id: value.id as i32,
			name: value.name,
		}
	}
}
