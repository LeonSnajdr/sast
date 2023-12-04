use prisma_client_rust::{Direction, QueryError};

use crate::contracts::placeholder_contracts::{
    CreatePlaceholderContract, UpdatePlaceholderContract,
};
use crate::prisma::{placeholder, project};
use crate::utils::db_utils::DbState;

pub async fn get_placeholders(
    db: DbState<'_>, project_id: String,
) -> Result<Vec<placeholder::Data>, QueryError> {
    let placeholders = db
        .placeholder()
        .find_many(vec![placeholder::project_id::equals(project_id)])
        .order_by(placeholder::order::order(Direction::Asc))
        .exec()
        .await;

    return placeholders;
}

pub async fn create_placeholder(
    db: DbState<'_>, create_contract: CreatePlaceholderContract,
) -> Result<placeholder::Data, QueryError> {
    return db
        .placeholder()
        .create(
            create_contract.order,
            create_contract.name,
            create_contract.value,
            project::id::equals(create_contract.project_id),
            vec![],
        )
        .exec()
        .await;
}

pub async fn update_placeholder(
    db: DbState<'_>, update_contract: UpdatePlaceholderContract,
) -> Result<placeholder::Data, QueryError> {
    return db
        .placeholder()
        .update(
            placeholder::id::equals(update_contract.id),
            vec![
                placeholder::order::set(update_contract.order),
                placeholder::value::set(update_contract.value),
            ],
        )
        .exec()
        .await;
}

pub async fn update_placeholders(
    db: DbState<'_>, update_contracts: Vec<UpdatePlaceholderContract>,
) -> Result<Vec<placeholder::Data>, QueryError> {
    let placeholder_updates = update_contracts.into_iter().map(|update_contract| {
        db.placeholder().update(
            placeholder::id::equals(update_contract.id),
            vec![
                placeholder::order::set(update_contract.order),
                placeholder::value::set(update_contract.value),
            ],
        )
    });

    return db._batch(placeholder_updates).await;
}

pub async fn delete_placeholder(
    db: DbState<'_>, placeholder_id: String,
) -> Result<placeholder::Data, QueryError> {
    return db
        .placeholder()
        .delete(placeholder::id::equals(placeholder_id))
        .exec()
        .await;
}
