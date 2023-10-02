use prisma_client_rust::QueryError;
use run_script::ScriptOptions;
use std::{thread, time};

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
    let task_set_to_start: full_set_contract::Data =
        task_set_repository::get_full_task_set(db, task_set_id)
            .await
            .unwrap()
            .unwrap();

    println!("Executed {}", task_set_to_start.project.id);

    for task in task_set_to_start.tasks {
        let mut command: String = task.command;

        for placeholder in task_set_to_start.project.placeholders.iter() {
            let placeholder_value = placeholder.value.as_str();

            command = command.replace(
                format!("<?sast {} ?>", placeholder.name).as_str(),
                placeholder_value,
            )
        }

        let keck: String = task.working_directory;

        println!("Command: {}", command);

        execute_command(keck, command);

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
