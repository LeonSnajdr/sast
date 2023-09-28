use prisma_client_rust::QueryError;
use run_script::ScriptOptions;

use crate::contracts::task_set_contracts::{full_set_contract, CreateTaskSetContract};
use crate::prisma::task_set;
use crate::repositories::task_set_repository;
use crate::utils::db_utils::DbState;

pub async fn create_task_set(
    db: DbState<'_>, create_contract: CreateTaskSetContract,
) -> Result<task_set::Data, QueryError> {
    return task_set_repository::create_task_set(db, create_contract).await;
}

pub async fn get_full_task_set(
    db: DbState<'_>, task_set_id: String,
) -> Result<Option<full_set_contract::Data>, QueryError> {
    return task_set_repository::get_full_task_set(db, task_set_id).await;
}

pub async fn start_task_set(db: DbState<'_>, task_set_id: String) -> Result<String, ()> {
    let task_set_to_start = task_set_repository::get_full_task_set(db, task_set_id)
        .await
        .unwrap();

    println!("Executed");

    Ok("Keckw".to_string())
}

fn execute_command(command: String) -> String {
    let options = ScriptOptions::new();
    let args = vec![];

    let denoted_command = format!(r#"{}"#, command);

    let (_, output, _) = run_script::run(&denoted_command, &args, &options).unwrap();

    return output;
}
