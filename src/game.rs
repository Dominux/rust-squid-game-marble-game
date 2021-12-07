const INIT_MARBLES_AMOUNT: u8 = 100;

#[derive(Debug, Default)]
pub struct Player {
    name: String,
    marbles_amount: u8,
}

#[derive(Debug, Default)]
pub struct Game {
    pub player1: Player,
    pub player2: Player,
	current_riddler: Option<Player>,
	current_guesser: Option<Player>,
}

impl Game {
    pub fn new(player_1_name: String, player_2_name: String) -> Game {
        let player1 = Player {
            name: player_1_name,
            marbles_amount: INIT_MARBLES_AMOUNT,
        };
        let player2 = Player {
            name: player_2_name,
            marbles_amount: INIT_MARBLES_AMOUNT,
        };
        Game { player1, player2, ..Default::default() }
    }

	// / Represent a single game move
	// pub fn game_move(&self) {

	// }
}
