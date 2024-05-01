
#[allow(unused)] // TODO: disable this

mod game;
use crate::game::Twirl;
use crate::game::Color::*;
use crate::game::TurnPhase::*;

fn print_game(game: &Twirl) {
    println!("  ======================");
    for line in game.to_string().lines() {
        println!("  {line}");
    }
    println!("  ======================");
}

fn main() {
    let mut game = Twirl::new(9);

    let turns = vec![
        (Black, Play, 0, 0, None),
        (Black, Spin, 0, 0, Some(4)),
        (Black, Play, 0, 0, None),
    ];

    println!("Initial game state:");
    print_game(&game);

    for turn in turns {
        if turn.1 == Play {
            println!("{:?} plays ({}, {})", turn.0, turn.2, turn.3);
            game.play(turn.0, turn.2, turn.3).unwrap();
        } else {
            println!("{:?} spins ({}, {}) size {}", turn.0, turn.2, turn.3, turn.4.unwrap());
            game.spin(turn.0, turn.2, turn.3, turn.4.unwrap()).unwrap();
        }

        print_game(&game);
    }
}
