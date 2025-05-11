use crate::project::project_contracts::{ProjectContract, ProjectUpdateContract};
use crate::project::project_models::{ProjectModel, ProjectUpdateModel};

impl ProjectContract {
	pub fn from(value: ProjectModel) -> Self {
		Self {
			id: value.id,
			name: value.name,
			favorite: value.favorite,
			quick_switch_keybind: value.quick_switch_keybind,
			date_created: value.date_created,
			date_last_opened: value.date_last_opened,
		}
	}
}

impl ProjectUpdateModel {
	pub fn from(value: ProjectUpdateContract) -> Self {
		Self {
			id: value.id,
			name: value.name,
			favorite: value.favorite,
			quick_switch_keybind: value.quick_switch_keybind,
		}
	}
}
