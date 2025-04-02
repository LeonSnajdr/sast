use crate::project::project_contracts::ProjectContract;
use crate::project::project_models::ProjectModel;

impl From<ProjectModel> for ProjectContract {
	fn from(value: ProjectModel) -> Self {
		Self {
			id: value.id,
			name: value.name,
			date_created: value.date_created,
			date_last_opened: value.date_last_opened,
		}
	}
}
