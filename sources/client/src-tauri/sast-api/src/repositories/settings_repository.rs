use prisma_client_rust::QueryError;

use crate::prisma::settings;
use crate::utils::db_utils::DbState;

pub async fn upsert_settings(db: &DbState<'_>) -> Result<settings::Data, QueryError> {
    // TODO do not hardcode default values instead use some kind of config

    let settings = db
        .settings()
        .upsert(
            settings::name::equals("default".to_string()),
            settings::create(
                "default".to_string(),
                true,
                "de".to_string(),
                "dark".to_string(),
                "c:\\".to_string(),
                vec![settings::default::set(true)],
            ),
            vec![],
        )
        .exec()
        .await;

    return settings;
}

pub async fn get_settings(db: &DbState<'_>) -> Result<Option<settings::Data>, QueryError> {
    return db
        .settings()
        .find_first(vec![settings::active::equals(true)])
        .exec()
        .await;
}
