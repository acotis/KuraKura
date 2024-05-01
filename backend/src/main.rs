
#[allow(unused)] // TODO: disable this

mod game;
use crate::game::Twirl;
use crate::game::Color::*;
use crate::game::TurnPhase::*;
use crate::game::SpinDirection::*;

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
        (Black, Play, 2, 2, None,    None),
        (Black, Spin, 2, 2, Some(1), Some(Right)),
        (White, Play, 2, 3, None,    None),
        (White, Spin, 0, 0, Some(9), Some(Right)),
    ];

    println!("Initial game state:");
    print_game(&game);

    for turn in turns {
        let result;

        let player = turn.0;
        let action = turn.1;
        let r = turn.2;
        let c = turn.3;

        if turn.1 == Play {
            print!("{player:?} plays ({r}, {c}).");
            result = game.play(player, r, c);
        } else {
            let size = turn.4.unwrap();
            let dir  = turn.5.unwrap();
            print!("{player:?} spins ({r}, {c}) size {size}, {dir:?}.");
            result = game.spin(player, r, c, size, dir);
        }

        println!(" Result = {result:?}");
        print_game(&game);
    }
}
