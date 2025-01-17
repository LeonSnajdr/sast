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

pub async fn get_all_placeholders() -> Result<Vec<PlaceholderContract>> {
    let placeholder_models = placeholder_repository::get_all_placeholders().await?;

    let placeholder_contracts: Vec<PlaceholderContract> = placeholder_models.into_iter().map(PlaceholderContract::from).collect();

    Ok(placeholder_contracts)
}
pub async fn get_placeholder(id: &Uuid) -> Result<PlaceholderContract> {
    let placeholder_model = placeholder_repository::get_placeholder(&id).await?;

    let placeholder_contract = PlaceholderContract::from(placeholder_model);

    Ok(placeholder_contract)
}
