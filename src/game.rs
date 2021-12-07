const INIT_MARBLES_AMOUNT: u8 = 100;

#[derive(Debug)]
struct Player {
    name: String,
    marbles_amount: u8,
}

#[derive(Debug)]
pub struct Game {
    player1: Box<Player>,
    player2: Box<Player>,
}

impl Game {
    pub fn new(player_1_name: String, player_2_name: String) -> Game {
        let player1 = Box::new(Player {
            name: player_1_name,
            marbles_amount: INIT_MARBLES_AMOUNT,
        });
        let player2 = Box::new(Player {
            name: player_2_name,
            marbles_amount: INIT_MARBLES_AMOUNT,
        });
        Game { player1, player2 }
    }
}
