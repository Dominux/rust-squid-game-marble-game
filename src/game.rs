use std::mem;

pub struct Player {
    pub name: String,
    marbles_amount: usize,
}

pub struct Game {
    pub state: State,
    riddler: Option<Player>,
    guesser: Option<Player>,
    riddler_parity: Option<Parity>,
    guessed_parity: Option<Parity>,
    bet: Option<usize>,
}

impl Game {
    pub fn new(player1_name: String, player2_name: String) -> Game {
        Game {
            state: State::PendingBoth,
            riddler: Some(Player {
                name: player1_name,
                marbles_amount: 100,
            }),
            guesser: Some(Player {
                name: player2_name,
                marbles_amount: 100,
            }),
            riddler_parity: None,
            guessed_parity: None,
            bet: None,
        }
    }

    pub fn riddler_move(&mut self, marbles_amount: usize) {
		// TODO: validation
        match self.state {
            State::PendingBoth => self.state = State::PendingGuesser,
            State::PendingRiddler => self.state = State::ReadyToDecision,
            _ => return,
        }

		self.riddler_parity = Some(Parity::from_number(marbles_amount));
    }

    pub fn guesser_move(&mut self, guessed_parity: &str, bet: usize) {
		// TODO: validation
		let parity = match Parity::from_string(guessed_parity) {
			Ok(r) => r,
			Err(_) => return,
		};

        match self.state {
            State::PendingBoth => self.state = State::PendingRiddler,
            State::PendingGuesser => self.state = State::ReadyToDecision,
            _ => return,
        }

		self.guessed_parity = Some(parity);
        self.bet = Some(bet);
    }

    pub fn decision_move(&mut self) {
        if !matches!(self.state, State::ReadyToDecision) {
            return;
        }

        // Deciding the winner
		let [mut winner, mut looser] = if self.riddler_parity == self.guessed_parity {
            [mem::take(&mut self.guesser).unwrap(), mem::take(&mut self.riddler).unwrap()]
        } else {
            [mem::take(&mut self.riddler).unwrap(), mem::take(&mut self.guesser).unwrap()]
        };

		looser.marbles_amount -= self.bet.unwrap();
		winner.marbles_amount += self.bet.unwrap();

		if looser.marbles_amount == 0 {
			// Ending the game
			self.state = State::GameOver;
			return;
		}

		// Setting roles
		self.riddler = Some(winner);
		self.guesser = Some(looser);

		// Nullabling some values
		self.riddler_parity = None;
		self.guessed_parity = None;
		self.bet = None;

		self.state = State::PendingBoth;
    }
}

#[derive(Debug)]
pub enum State {
    PendingBoth,
    PendingRiddler,
    PendingGuesser,
    ReadyToDecision,
    GameOver,
}

#[derive(PartialEq)]
enum Parity {
	Even,
	Odd
}

impl Parity {
	fn from_number(n: usize) -> Parity {
		match n % 2 {
			1 => Parity::Odd,
			_ => Parity::Even,
		}
	}

	fn from_string(s: &str) -> Result<Parity, ()> {
		match s.to_lowercase().as_str() {
			"even" => Ok(Parity::Even),
			"odd" => Ok(Parity::Odd),
			_ => Err(())
		}
	}
}
