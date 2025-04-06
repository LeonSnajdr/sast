use serde::Serialize;
use specta::Type;

#[derive(Debug, Serialize, Type)]
pub enum Error {
	Db(String),
	AlreadyExists,
	NotExists,
	InvalidStatus,
	EventEmit,
	Failed,
}
