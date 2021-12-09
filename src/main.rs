use crate::game::{Game, Player};

mod game;

fn main() {
	let player1 = Player::new();
	let player2 = Player::new();
	let mut game = Game::start(&mut player1, &mut player2);
	
	loop {
		player1.make_move(&mut game);
		player2.make_move(&mut game);
		match game.end_move() {
			None => (),
			_ => break,
		}
	}
}
