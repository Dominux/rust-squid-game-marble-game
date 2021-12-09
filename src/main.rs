use crate::game::{Game};

mod game;

fn main() {
	let (player1, player2, mut game) = Game::start();
	
	loop {
		player1.make_move(&mut game);
		player2.make_move(&mut game);
		match game.end_move() {
			None => (),
			_ => break,
		}
	}
}
