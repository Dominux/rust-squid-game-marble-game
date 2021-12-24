use teloxide::prelude::*;
use frunk::Generic;
use derive_more::From;

/////////////////////////////////////////////////////////////
/// Dialog itself
/////////////////////////////////////////////////////////////

#[derive(Transition, Clone, From)]
pub enum Dialog {
	Start(StartState),
	RecieveData(RecieveDataState),
	// Decision(DecisionState),
	// GameOver(GameOverState),
}

impl Default for Dialog {
	fn default() -> Self {
		Self::Start(StartState{..Default::default()})
	}
}

/////////////////////////////////////////////////////////////
/// States
/////////////////////////////////////////////////////////////

#[derive(Clone, Generic, Default)]
pub struct StartState {
	pub name: Option<String>,
}

#[derive(Clone)]
pub struct RecieveDataState;
