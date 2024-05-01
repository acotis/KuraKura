
#[allow(unused)] // TODO: disable this

mod game;
use crate::game::Twirl;
use crate::game::Color::*;
use crate::game::TurnPhase::*;

fn print_game(game: &Twirl) {
    let string = game.to_string();
    let lines = string.lines();
    let width = lines.clone().fold(0, |best, next| best.max(next.len()));

    print!("  ┌─");
    for _ in 0..width {print!("─")};
    print!("─┐\n");

    for line in lines {
        print!("  │ {line}");
        for _ in 0..(width - line.len()) {print!(" ");}
        print!(" │\n");
    }

    print!("  └─");
    for _ in 0..width {print!("─")};
    print!("─┘\n");
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
        let result;

        if turn.1 == Play {
            print!("{:?} plays ({}, {}).", turn.0, turn.2, turn.3);
            result = game.play(turn.0, turn.2, turn.3);
        } else {
            print!("{:?} spins ({}, {}) size {}.", turn.0, turn.2, turn.3, turn.4.unwrap());
            result = game.spin(turn.0, turn.2, turn.3, turn.4.unwrap());
        }

        println!(" Result = {result:?}");
        print_game(&game);
    }
}
