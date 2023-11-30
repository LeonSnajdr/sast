use prisma_client_rust::QueryError;
use run_script::ScriptOptions;
use std::{thread, time};

use crate::contracts::task_set_contracts::{
    full_task_set_contract, project_task_set_contract, CreateTaskSetContract, UpdateTaskSetContract,
};
use crate::prisma::task_set;
use crate::repositories::task_set_repository;
use crate::utils::db_utils::DbState;

#[tauri::command]
#[specta::specta]
pub async fn get_task_sets(
    db: DbState<'_>, project_id: String,
) -> Result<Vec<task_set::Data>, QueryError> {
    return task_set_repository::get_task_sets(db, project_id).await;
}

#[tauri::command]
#[specta::specta]
pub async fn create_task_set(
    db: DbState<'_>, create_contract: CreateTaskSetContract,
) -> Result<full_task_set_contract::Data, QueryError> {
    return task_set_repository::create_task_set(db, create_contract).await;
}

#[tauri::command]
#[specta::specta]
pub async fn update_task_set(
    db: DbState<'_>, update_contract: UpdateTaskSetContract,
) -> Result<full_task_set_contract::Data, QueryError> {
    return task_set_repository::update_task_set(db, update_contract).await;
}

#[tauri::command]
#[specta::specta]
pub async fn update_task_sets(
    db: DbState<'_>, update_contracts: Vec<UpdateTaskSetContract>,
) -> Result<Vec<task_set::Data>, QueryError> {
    return task_set_repository::update_task_sets(db, update_contracts).await;
}

#[tauri::command]
#[specta::specta]
pub async fn delete_task_set(
    db: DbState<'_>, task_set_id: String,
) -> Result<full_task_set_contract::Data, QueryError> {
    return task_set_repository::delete_task_set(db, task_set_id).await;
}

#[tauri::command]
#[specta::specta]
pub async fn start_task_set(db: DbState<'_>, task_set_id: String) -> Result<String, ()> {
    let task_set_to_start: project_task_set_contract::Data =
        task_set_repository::get_project_task_set(db, task_set_id)
            .await
            .unwrap()
            .unwrap();

    println!("Executed {}", task_set_to_start.project.id);

    for task in task_set_to_start.tasks {
        let mut command: String = task.command;
        let mut working_directory: String = task.working_directory;

        for placeholder in task_set_to_start.project.placeholders.iter() {
            command = command.replace(
                format!("@{}", placeholder.name).as_str(),
                placeholder.value.as_str(),
            );

            working_directory = working_directory.replace(
                format!("@{}", placeholder.name).as_str(),
                placeholder.value.as_str(),
            );
        }

        println!("Command: {}", command);

        execute_command(working_directory, command);

        let delay = u64::try_from(task.delay).unwrap();
        thread::sleep(time::Duration::from_millis(delay));
    }

    Ok("Keckw".to_string())
}

fn execute_command(working_directory: String, command: String) {
    let mut options = ScriptOptions::new();

    options.working_directory = Some(std::path::PathBuf::from(working_directory));

    let args = vec![];

    let denoted_command = format!(r#"{}"#, command);

    let (_, output, _) = run_script::run(&denoted_command, &args, &options).unwrap();

    println!("Output: {}", output);
}
