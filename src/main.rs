use crate::game::{Game};

mod game;

fn main() {
	let mut game = Game::new(String::from("Lol"), String::from("Kek"));
	game.riddler_move(25);
	game.guesser_move("even", 12);
	game.decision_move();
	println!("{:?}", game.state);
}
