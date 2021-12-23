use teloxide::prelude::*;

#[derive(Clone)]
struct StartState;

#[teloxide(subtransition)]
async fn start(
    _state: StartState,
    cx: TransitionIn<AutoSend<Bot>>,
    _ans: String,
) -> TransitionOut<Dialog> {
    cx.answer("Let's start! What's your full name?").await?;
    next(StartState)
}

#[derive(Transition, From)]
enum Dialog {
	Start(StartState),
	RecieveName(StartState),
}

pub async fn main() {
    teloxide::enable_logging!();
    log::info!("Starting dices_bot...");

    let bot = Bot::from_env().auto_send();

    teloxide::repl(bot, |message| async move {
        message.answer_dice().await?;
        respond(())
    })
    .await;
}
