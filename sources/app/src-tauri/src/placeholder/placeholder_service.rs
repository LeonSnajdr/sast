use chrono::Utc;
use uuid::Uuid;

use crate::prelude::*;
use crate::placeholder::placeholder_contracts::{CreatePlaceholderContract, PlaceholderContract};
use crate::placeholder::placeholder_repository;

pub async fn create_placeholder(create_placeholder_contract: &CreatePlaceholderContract) -> Result<PlaceholderContract> {
    let date_created = Utc::now();
    let date_last_updated = Utc::now();

    let placeholder_model = placeholder_repository::create_placeholder(&create_placeholder_contract.project_id, &create_placeholder_contract.name, &create_placeholder_contract.value, &date_created, &date_last_updated).await?;

    let placeholder_contract = PlaceholderContract::from(placeholder_model);

    Ok(placeholder_contract)
}

pub async fn get_global_placeholders() -> Result<Vec<PlaceholderContract>> {

    let empty_project_id: Option<Uuid> = None;

    let placeholder_contracts = get_project_placeholders(&empty_project_id).await?;

    Ok(placeholder_contracts)
}

pub async fn get_project_placeholders(project_id: &Option<Uuid>) -> Result<Vec<PlaceholderContract>> {

    let placeholder_models = placeholder_repository::get_placeholders(&project_id).await?;

    let placeholder_contracts = placeholder_models.into_iter().map(PlaceholderContract::from).collect();

    Ok(placeholder_contracts)
}
pub async fn get_placeholder(id: &Uuid) -> Result<PlaceholderContract> {
    let placeholder_model = placeholder_repository::get_placeholder(&id).await?;

    let placeholder_contract = PlaceholderContract::from(placeholder_model);

    Ok(placeholder_contract)
}
