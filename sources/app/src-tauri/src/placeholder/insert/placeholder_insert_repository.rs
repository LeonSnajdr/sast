use uuid::Uuid;

use crate::db;
use crate::placeholder::placeholder_enums::PlaceholderVisibility;
use crate::placeholder::insert::placeholder_insert_enums::PlaceholderInsertTileKind;
use crate::placeholder::insert::placeholder_insert_models::{PlaceholderInsertTileFilterModel, PlaceholderInsertTileModel, PlaceholderInsertTileResultModel};
use crate::prelude::*;

pub async fn placeholder_insert_tile_update_many(create_models: Vec<PlaceholderInsertTileModel>, delete_filter: PlaceholderInsertTileFilterModel) -> Result<()> {

	let mut tx = db::get_pool().begin().await.map_err(|_| Error::Db)?;

	sqlx::query!(
		r#"--sql
			delete
			from placeholder_insert_tile
			where
				task_command_id is $1 and
				task_working_dir_id is $2
		"#,
		delete_filter.task_command_id,
		delete_filter.task_working_dir_id
	)
	.execute(&mut *tx)
	.await
	.map_err(|_| Error::Db)?;

	for create_model in create_models.iter() {
		sqlx::query!(
			r#"--sql
				insert into placeholder_insert_tile
					(id, placeholder_id, task_command_id, task_working_dir_id, kind, position, text_value)
				values
					($1, $2, $3, $4, $5, $6, $7)
			"#,
			create_model.id,
			create_model.placeholder_id,
			create_model.task_command_id,
			create_model.task_working_dir_id,
			create_model.kind,
			create_model.position,
			create_model.text_value
		)
		.execute(&mut *tx)
		.await
		.map_err(|_| Error::Db)?;
	}

	tx.commit().await.map_err(|_| Error::Db)?;

    Ok(())
}

pub async fn placeholder_insert_tile_get_many(filter: PlaceholderInsertTileFilterModel) -> Result<Vec<PlaceholderInsertTileResultModel>> {
	let placeholder_insert_tiles = sqlx::query_as!(
		PlaceholderInsertTileResultModel,
		r#"--sql
            select
            	pit.kind as "kind: PlaceholderInsertTileKind",
            	pit.position,
            	pit.text_value,
            	pit.placeholder_id as "placeholder_id: Uuid",
            	p.name as placeholder_name,
            	p.visibility as "placeholder_visibility: PlaceholderVisibility"
            from placeholder_insert_tile as pit
            left join placeholder as p on p.id = pit.placeholder_id
            where
            	pit.task_command_id is $1 and
            	pit.task_working_dir_id is $2
        "#,
		filter.task_command_id,
		filter.task_working_dir_id
	)
		.fetch_all(db::get_pool())
		.await
		.map_err(|_| Error::Db)?;

	Ok(placeholder_insert_tiles)
}