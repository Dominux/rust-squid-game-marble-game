use std::io::{self, BufRead};
use std::mem;

const INIT_MARBLES_AMOUNT: u8 = 100;

#[derive(Debug)]
pub enum PlayerRole {
    Riddler,
    Guesser,
}

type MarblesAmount = u8;

#[derive(Debug, Default)]
pub struct Player {
    pub name: String,
    pub marbles_amount: MarblesAmount,
    role: Option<PlayerRole>,
}

impl Player {
    pub fn new() -> Player {
        // getting user input
        let mut name = String::new();
        io::stdin().lock().read_line(&mut name).unwrap();

        Player {
            name: name,
            marbles_amount: 100,
            role: None,
        }
    }

    pub fn make_move(&self, game: &mut Game) {
        match self.role {
            Some(PlayerRole::Riddler) => self.make_move_as_riddler(game),
            Some(PlayerRole::Guesser) => self.make_move_as_guesser(game),
            None => (),
        }
    }

    fn make_move_as_riddler(&self, game: &mut Game) {
        // telling the riddler what to do
        println!("Sup, nibba! Ya gotta choose even or odd amount of your marbles:");

        let number = loop {
            // getting user input
            let mut line = String::new();
            io::stdin().lock().read_line(&mut line).unwrap();
            let number = line.parse::<MarblesAmount>().unwrap();

            // validating
            if number > MarblesAmount::MIN && number <= self.marbles_amount {
                break number;
            } else {
                println!("Invalid amount, ya dumba, write again:")
            }
        };

        game.parety = match number % 2 {
            0 => Some(Parety::Even),
            1 => Some(Parety::Odd),
            _ => None,
        }
    }

    fn make_move_as_guesser(&self, game: &mut Game) {
        // telling the guesser what to do
        println!("Sub nibba! Ya gotta guess whether \"even\" or \"odd\" the riddler chose:");

        game.guessed_parety = loop {
            // getting user input
            let mut line = String::new();
            io::stdin().lock().read_line(&mut line).unwrap();

            // validating
            match line.to_lowercase().as_str() {
                "even" => break Some(Parety::Even),
                "odd" => break Some(Parety::Odd),
                _ => println!("Invalid word, dumba, write \"even\" or \"odd\":"),
            }
        };

        // asking the guesser how many marbles he is ready to bet
        println!("How many of ya marbles ya gonna bet:");

        game.bet = Some(loop {
            // getting user input
            let mut line = String::new();
            io::stdin().lock().read_line(&mut line).unwrap();
            let number = line.parse::<MarblesAmount>().unwrap();

            // validating
            if number > MarblesAmount::MIN && number <= self.marbles_amount {
                break number;
            } else {
                println!("Invalid amount, ya dumba, write again:")
            }
        });
    }
}

#[derive(Debug)]
pub enum Parety {
    Even,
    Odd,
}

#[derive(Debug)]
pub struct Game<'p> {
    pub player1: &'p mut Player,
    pub player2: &'p mut Player,
    parety: Option<Parety>,
    guessed_parety: Option<Parety>,
    bet: Option<MarblesAmount>,
}

impl<'p> Game<'p> {
    pub fn start(player1: &'p mut Player, player2: &'p mut Player) -> Game<'p> {
        Game {
            player1: player1,
            player2: player2,
            parety: None,
            guessed_parety: None,
            bet: None,
        }
    }

    /// Defining a winner and a looser and taking a bet transfer from the looser to the winner
    pub fn end_move(&mut self) -> Option<EndGameResult> {
        // defining who is riddler and who is guesser
        let (riddler, guesser) = match (&self.player1.role, &self.player2.role) {
            (Some(PlayerRole::Riddler), Some(PlayerRole::Guesser)) => {
                (self.player1, self.player2)
            }
            (Some(PlayerRole::Guesser), Some(PlayerRole::Riddler)) => {
                (self.player2, self.player1)
            }
            _ => panic!("Ya dumba, some player has no his role!"),
        };

        // defining a winner and a looser
        let mut winner: &Player;
        let mut looser: &Player;
        match &self.parety {
            Some(parety) => match &self.guessed_parety {
                Some(guessed_parety) => {
                    if mem::discriminant(parety) == mem::discriminant(guessed_parety) {
                        winner = guesser;
                        looser = riddler;
                    } else {
                        winner = riddler;
                        looser = guesser;
                    }
                }
                None => panic!("Ya dumba, the guesser hasn't chosen the parety yet!"),
            },
            None => panic!("Ya dumba, the riddler hasn't chosen the parety yet!"),
        }

        // taking the bet transfer from the looser to the winner
        match self.bet {
            Some(bet) => {
                looser.marbles_amount -= bet;
                winner.marbles_amount += bet;
            }
            None => panic!("Ya dumba, bet is undefined!"),
        }

        // if looser is lack of marbles, then the game is over
        if looser.marbles_amount == 0 {
            return Some(EndGameResult {
                winner: winner,
                looser: looser,
            });
        }

        // Turning move belonged fields to None
        self.parety = None;
        self.guessed_parety = None;
        self.bet = None;

        None
    }
}

struct EndGameResult<'p> {
    winner: &'p Player,
    looser: &'p Player,
}
