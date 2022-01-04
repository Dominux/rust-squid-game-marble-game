use std::mem;

use crate::errors::GameError;

pub struct Player {
    pub name: String,
    marbles_amount: usize,
    role: Role,
}

enum Role {
    Riddler,
    Guesser,
}

pub struct Game {
    pub state: State,
    player1: Option<Player>,
    player2: Option<Player>,
    riddler_parity: Option<Parity>,
    guessed_parity: Option<Parity>,
    bet: Option<usize>,
}

impl Game {
    pub fn new(player1_name: String, player2_name: String) -> Game {
        Game {
            state: State::PendingBoth,
            player1: Some(Player {
                name: player1_name,
                marbles_amount: 100,
                role: Role::Riddler,
            }),
            player2: Some(Player {
                name: player2_name,
                marbles_amount: 100,
                role: Role::Guesser,
            }),
            riddler_parity: None,
            guessed_parity: None,
            bet: None,
        }
    }

    pub fn riddler_move(&mut self, marbles_amount: usize) -> Result<(), GameError> {
        // Validation
        let riddler = if matches!(self.player1.as_ref().unwrap().role, Role::Riddler) {
            self.player1.as_ref().unwrap()
        } else {
            self.player2.as_ref().unwrap()
        };
        if marbles_amount > riddler.marbles_amount {
            return Err(GameError::ValidationError(format!(
                "Marbles amount is to big for you, you have only {}",
                riddler.marbles_amount
            )));
        }

        match self.state {
            State::PendingBoth => self.state = State::PendingGuesser,
            State::PendingRiddler => self.state = State::ReadyToDecision,
            _ => {
                return Err(GameError::WrongStateError(String::from(
                    "It is not the riddler move's time",
                )))
            }
        }

        self.riddler_parity = Some(Parity::from_number(marbles_amount));
        Ok(())
    }

    pub fn guesser_move(&mut self, guessed_parity: &str, bet: usize) -> Result<(), GameError> {
        // Validation
        let [guesser, riddler] = if matches!(self.player1.as_ref().unwrap().role, Role::Guesser) {
            [
                self.player1.as_ref().unwrap(),
                self.player2.as_ref().unwrap(),
            ]
        } else {
            [
                self.player2.as_ref().unwrap(),
                self.player1.as_ref().unwrap(),
            ]
        };
        if bet > guesser.marbles_amount {
            return Err(GameError::ValidationError(format!(
                "Bet is too big for you, you have only {}",
                guesser.marbles_amount
            )));
        }
        if bet > riddler.marbles_amount {
            return Err(GameError::ValidationError(format!(
                "Bet is too big for riddler, he has only {}",
                riddler.marbles_amount
            )));
        }
        let guessed_parity = Parity::from_string(guessed_parity).map_err(|_e| {
            GameError::ValidationError("input is not \"even\" or \"odd\"".to_string())
        })?;

        match self.state {
            State::PendingBoth => self.state = State::PendingRiddler,
            State::PendingGuesser => self.state = State::ReadyToDecision,
            _ => {
                return Err(GameError::WrongStateError(String::from(
                    "It is not the guesser move's time",
                )))
            }
        }

        self.guessed_parity = Some(guessed_parity);
        self.bet = Some(bet);
        Ok(())
    }

    pub fn decision_move(&mut self) -> Result<(), GameError> {
        if !matches!(self.state, State::ReadyToDecision) {
            return Err(GameError::WrongStateError(String::from(
                "It is not the time to decide winner/looser",
            )));
        }

        let [guesser, riddler] = if matches!(self.player1.as_ref().unwrap().role, Role::Guesser) {
            [
                mem::take(&mut self.player1).unwrap(),
                mem::take(&mut self.player2).unwrap(),
            ]
        } else {
            [
                mem::take(&mut self.player2).unwrap(),
                mem::take(&mut self.player1).unwrap(),
            ]
        };

        // Deciding the winner
        let [mut winner, mut looser] = if self.riddler_parity == self.guessed_parity {
            [guesser, riddler]
        } else {
            [riddler, guesser]
        };

        looser.marbles_amount -= self.bet.unwrap();
        winner.marbles_amount += self.bet.unwrap();

        if looser.marbles_amount == 0 {
            // Ending the game
            self.state = State::GameOver;
            return Ok(());
        }

        // Setting roles
        mem::swap(&mut winner.role, &mut looser.role);
        self.player1 = Some(winner);
        self.player2 = Some(looser);

        // Nullabling some values
        self.riddler_parity = None;
        self.guessed_parity = None;
        self.bet = None;

        self.state = State::PendingBoth;

        Ok(())
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
    Odd,
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
            _ => Err(()),
        }
    }
}
