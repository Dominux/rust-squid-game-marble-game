use crate::game::{Game};

mod game;

fn main() {
	let game = Game::start();
	
	loop {
		game.player1.make_move(&mut game);
		game.player2.make_move(&mut game);
		match game.end_move() {
			None => (),
			_ => break,
		}
	}
}
