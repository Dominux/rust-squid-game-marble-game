use teloxide::prelude::*;
use frunk::{Generic, from_generic};

/////////////////////////////////////////////////////////////
/// Dialog itself
/////////////////////////////////////////////////////////////

#[derive(Transition, Clone)]
pub enum Dialog {
	Start(StartState),
	RecieveData(RecieveDataState),
	Decision(DecisionState),
	GameOver(GameOverState),
}

impl Default for Dialog {
	fn default() -> Self {
		Self::Start(StartState)
	}
}

impl From

/////////////////////////////////////////////////////////////
/// States
/////////////////////////////////////////////////////////////

#[derive(Clone, Generic)]
pub struct StartState;

#[teloxide(subtransition)]
async fn start(
	state: StartState,
	cx: TransitionIn<AutoSend<Bot>>,
	ans: String,
) -> TransitionOut<Dialog> {
	cx.answer("Sup nibba\nYou know the rules BTW, so what's ur fucking name?").await?;
	next(RecieveDataState)
}

#[derive(Clone)]
pub struct RecieveDataState;

#[teloxide(subtransition)]
async fn start(
	state: RecieveDataState,
	cx: TransitionIn<AutoSend<Bot>>,
	ans: String,
) -> TransitionOut<Dialog> {
	cx.answer("Sup nibba\nYou know the rules BTW, so").await?;
	next()
}
