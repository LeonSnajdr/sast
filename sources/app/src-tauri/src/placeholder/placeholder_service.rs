use chrono::Utc;
use uuid::Uuid;

use crate::prelude::*;
use crate::placeholder::placeholder_contracts::{PlaceholderCreateContract, PlaceholderContract};
use crate::placeholder::placeholder_repository;

pub async fn placeholder_create(placeholder_create_contract: &PlaceholderCreateContract) -> Result<PlaceholderContract> {
    let date_created = Utc::now();
    let date_last_updated = Utc::now();

    let placeholder_model = placeholder_repository::placeholder_create(&placeholder_create_contract.project_id, &placeholder_create_contract.name, &placeholder_create_contract.value, &date_created, &date_last_updated).await?;

    let placeholder_contract = PlaceholderContract::from(placeholder_model);

    Ok(placeholder_contract)
}

pub async fn placeholder_get_all_global() -> Result<Vec<PlaceholderContract>> {

    let empty_project_id: Option<Uuid> = None;

    let placeholder_contracts = placeholder_get_all_project(&empty_project_id).await?;

    Ok(placeholder_contracts)
}

pub async fn placeholder_get_all_project(project_id: &Option<Uuid>) -> Result<Vec<PlaceholderContract>> {

    let placeholder_models = placeholder_repository::placeholder_get_many(&project_id).await?;

    let placeholder_contracts = placeholder_models.into_iter().map(PlaceholderContract::from).collect();

    Ok(placeholder_contracts)
}
pub async fn placeholder_get_one(id: &Uuid) -> Result<PlaceholderContract> {
    let placeholder_model = placeholder_repository::placeholder_get_one(&id).await?;

    let placeholder_contract = PlaceholderContract::from(placeholder_model);

    Ok(placeholder_contract)
}
