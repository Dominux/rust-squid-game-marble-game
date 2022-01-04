#[derive(thiserror::Error, Debug)]
pub enum GameError {
	#[error("{0}")]
	ValidationError(String),
	#[error("{0}")]
	WrongStateError(String),
}
