use crate::placeholder::placeholder_contracts::PlaceholderContract;
use crate::placeholder::placeholder_models::PlaceholderModel;

impl From<PlaceholderModel> for PlaceholderContract {
    fn from(value: PlaceholderModel) -> Self {
        Self {
            id: value.id,
            project_id: value.project_id,
            name: value.name,
            value: value.value,
            date_created: value.date_created,
            date_last_updated: value.date_last_updated
        }
    }
}
