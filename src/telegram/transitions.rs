use teloxide::prelude::*;

use crate::telegram::states::*;

#[teloxide(subtransition)]
async fn start(
    state: StartState,
    cx: TransitionIn<AutoSend<Bot>>,
    _ans: String,
) -> TransitionOut<Dialog> {
    match state.name {
        Some(name) => println!("{}", name),
        None => {
            cx.answer("Sup nibba\nYou know the rules BTW, so what's ur fucking name?")
                .await?;
        }
    }
    next(RecieveDataState)
}

#[teloxide(subtransition)]
async fn start(
    _state: RecieveDataState,
    cx: TransitionIn<AutoSend<Bot>>,
    ans: String,
) -> TransitionOut<Dialog> {
    let name = ans;

    cx.answer(format!(
        "Ur name is so foolish, just imagine how it sounds like: \"{}\"",
        &name
    ))
    .await?;
    cx.answer("Try again, write ur name").await?;

    next(StartState { name: Some(name) })
}
