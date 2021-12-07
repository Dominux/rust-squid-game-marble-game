use crate::game::{Game};

mod game;

fn main() {
	let game = Game::new(String::from("lol"), String::from("kek"));
    println!("{:?}", game.player1);
}
