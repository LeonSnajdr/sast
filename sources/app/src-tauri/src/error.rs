use serde::Serialize;
use specta::Type;

#[derive(Debug, Serialize, Type)]
pub enum Error {
	Db,
	AlreadyExists,
	NotExists,
	Failed,
}
