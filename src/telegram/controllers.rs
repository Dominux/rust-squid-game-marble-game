use teloxide::prelude::*;

use crate::telegram::states::*;

pub async fn main() {
    teloxide::enable_logging!();
    log::info!("Starting dices_bot...");

    let bot = Bot::from_env().auto_send();

	teloxide::dialogues_repl(bot, |message, dialog| async move {
		handle_message(message, dialog).await.expect("SOmethng wrong with bot")
	}).await;
}

async fn handle_message(
    cx: UpdateWithCx<AutoSend<Bot>, Message>,
    dialog: Dialog,
) -> TransitionOut<Dialog> {
    match cx.update.text().map(ToOwned::to_owned) {
        None => {
            cx.answer("Send me a text message.").await?;
            next(dialog)
        }
        Some(ans) => dialog.react(cx, ans).await,
    }
}
