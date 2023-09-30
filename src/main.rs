use clap::Parser;

use rand::Rng;

struct Tile {
    name: Option<String>,
    jump: Option<u32>,
}

impl Tile {
    fn new(name: Option<String>, jump: Option<u32>) -> Tile {
        Tile { name, jump }
    }
}

struct Player {
    name: String,
    position: u32,
}

impl Player {
    fn new(name: String) -> Player {
        Player { name, position: 0 }
    }
}

struct Die {
    sides: u32,
}

impl Die {
    fn new(sides: u32) -> Die {
        Die { sides }
    }

    fn roll(&self) -> u32 {
        let mut rng = rand::thread_rng();
        rng.gen_range(1..self.sides + 1)
    }
}

#[derive(Parser)]
struct Args {
    /// Name of the player. Use multiple times for multiple players.
    #[arg(long = "player", short = 'p', default_values_t = vec!["Player".to_string()])]
    players: Vec<String>,

    /// Number of sides on the die.
    #[arg(long = "die", short = 'd', default_value_t = 6)]
    die: u32,
}

fn main() {
    let args = Args::parse();

    let mut board = Vec::new();
    board.push(Tile::new(Some("Start".to_string()), None));
    board.push(Tile::new(Some("Zebra".to_string()), Some(38)));
    board.push(Tile::new(None, None));
    board.push(Tile::new(None, None));
    board.push(Tile::new(Some("Ostrich".to_string()), Some(14)));
    board.push(Tile::new(None, None));
    board.push(Tile::new(None, None));
    board.push(Tile::new(None, None));
    board.push(Tile::new(None, None));
    board.push(Tile::new(Some("Elephant".to_string()), Some(31)));
    board.push(Tile::new(None, None));
    board.push(Tile::new(None, None));
    board.push(Tile::new(None, None));
    board.push(Tile::new(None, None));
    board.push(Tile::new(None, None));
    board.push(Tile::new(None, None));
    board.push(Tile::new(Some("Alligator".to_string()), Some(6)));
    board.push(Tile::new(None, None));
    board.push(Tile::new(None, None));
    board.push(Tile::new(None, None));
    board.push(Tile::new(None, None));
    board.push(Tile::new(Some("Chimp".to_string()), Some(42)));
    board.push(Tile::new(None, None));
    board.push(Tile::new(None, None));
    board.push(Tile::new(None, None));
    board.push(Tile::new(None, None));
    board.push(Tile::new(None, None));
    board.push(Tile::new(None, None));
    board.push(Tile::new(Some("Fallow deer".to_string()), Some(84)));
    board.push(Tile::new(None, None));
    board.push(Tile::new(None, None));
    board.push(Tile::new(None, None));
    board.push(Tile::new(None, None));
    board.push(Tile::new(None, None));
    board.push(Tile::new(None, None));
    board.push(Tile::new(None, None));
    board.push(Tile::new(Some("Chipmunk".to_string()), Some(44)));
    board.push(Tile::new(None, None));
    board.push(Tile::new(None, None));
    board.push(Tile::new(None, None));
    board.push(Tile::new(None, None));
    board.push(Tile::new(None, None));
    board.push(Tile::new(None, None));
    board.push(Tile::new(None, None));
    board.push(Tile::new(None, None));
    board.push(Tile::new(None, None));
    board.push(Tile::new(None, None));
    board.push(Tile::new(Some("Hippo".to_string()), Some(26)));
    board.push(Tile::new(None, None));
    board.push(Tile::new(Some("Bear".to_string()), Some(11)));
    board.push(Tile::new(None, None));
    board.push(Tile::new(Some("Giraffe".to_string()), Some(67)));
    board.push(Tile::new(None, None));
    board.push(Tile::new(None, None));
    board.push(Tile::new(None, None));
    board.push(Tile::new(None, None));
    board.push(Tile::new(Some("Impala".to_string()), Some(53)));
    board.push(Tile::new(None, None));
    board.push(Tile::new(None, None));
    board.push(Tile::new(None, None));
    board.push(Tile::new(None, None));
    board.push(Tile::new(None, None));
    board.push(Tile::new(Some("Tiger".to_string()), Some(19)));
    board.push(Tile::new(None, None));
    board.push(Tile::new(Some("Cheetah".to_string()), Some(60)));
    board.push(Tile::new(None, None));
    board.push(Tile::new(None, None));
    board.push(Tile::new(None, None));
    board.push(Tile::new(None, None));
    board.push(Tile::new(None, None));
    board.push(Tile::new(None, None));
    board.push(Tile::new(Some("Gazelle".to_string()), Some(91)));
    board.push(Tile::new(None, None));
    board.push(Tile::new(None, None));
    board.push(Tile::new(None, None));
    board.push(Tile::new(None, None));
    board.push(Tile::new(None, None));
    board.push(Tile::new(None, None));
    board.push(Tile::new(None, None));
    board.push(Tile::new(None, None));
    board.push(Tile::new(Some("Rabbit".to_string()), Some(100)));
    board.push(Tile::new(None, None));
    board.push(Tile::new(None, None));
    board.push(Tile::new(None, None));
    board.push(Tile::new(None, None));
    board.push(Tile::new(None, None));
    board.push(Tile::new(None, None));
    board.push(Tile::new(Some("Lion".to_string()), Some(24)));
    board.push(Tile::new(None, None));
    board.push(Tile::new(None, None));
    board.push(Tile::new(None, None));
    board.push(Tile::new(None, None));
    board.push(Tile::new(None, None));
    board.push(Tile::new(Some("Jackal".to_string()), Some(73)));
    board.push(Tile::new(None, None));
    board.push(Tile::new(Some("Gorilla".to_string()), Some(75)));
    board.push(Tile::new(None, None));
    board.push(Tile::new(None, None));
    board.push(Tile::new(Some("Rhino".to_string()), Some(78)));
    board.push(Tile::new(None, None));
    board.push(Tile::new(None, None));

    let die = Die::new(args.die);

    let mut players = Vec::new();
    for name in args.players {
        players.push(Player::new(name));
    }

    loop {
        for player in &mut players {
            println!("\n{}'s turn at {}.", player.name, player.position);

            let roll = die.roll();
            println!("Rolled a {}.", roll);

            player.position += roll;

            if player.position >= board.len() as u32 {
                println!("{} wins!", player.name);
                return;
            }

            if board[player.position as usize].name.is_some() {
                println!(
                    "Landed on {}!",
                    board[player.position as usize].name.as_ref().unwrap()
                );
            }

            if board[player.position as usize].jump.is_some() {
                println!(
                    "Jumping to {}.",
                    board[player.position as usize].jump.unwrap()
                );
                player.position = board[player.position as usize].jump.unwrap();
            }

            println!("Position is now {}.", player.position);
        }
    }
}
