#[derive(Debug, thiserror::Error)]
pub enum Error {
	#[error("Generic {0}")]
	Generic(String),
	#[error(transparent)]
	Tauri(#[from] tauri::Error),
	#[error(transparent)]
	Db(#[from] sqlx::Error),
}

impl serde::Serialize for Error {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::ser::Serializer,
	{
		serializer.serialize_str(self.to_string().as_ref())
	}
}
