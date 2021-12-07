const INIT_MARBLES_AMOUNT: u8 = 100;

#[derive(Debug)]
pub enum PlayerMovingPosition {
	Riddler,
	Guesser,
}


#[derive(Debug)]
pub struct Player {
    pub name: String,
    pub marbles_amount: u8,
	pub moving_position: PlayerMovingPosition,
}

// impl Player {
// 	pub fn make_move(&self, game: &Game) {
// 	}
// }

#[derive(Debug)]
pub struct Game {
    pub player1: Player,
    pub player2: Player,
}

impl Game {
    pub fn new(player_1_name: String, player_2_name: String) -> Game {
        let player1 = Player {
            name: player_1_name,
            marbles_amount: INIT_MARBLES_AMOUNT,
			moving_position: PlayerMovingPosition::Riddler,
        };
        let player2 = Player {
            name: player_2_name,
            marbles_amount: INIT_MARBLES_AMOUNT,
			moving_position: PlayerMovingPosition::Guesser,
        };
        Game { player1, player2 }
    }
}
