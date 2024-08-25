
use insta::Settings;
use insta::internals::SettingsBindDropGuard;

use kurakura::Player::{self, *};
use kurakura::Game;
use kurakura::Turn;
use kurakura::SpinDirection::{self, *};

#[test]
fn original_test() {
    let _guard = configure_insta();
    let game = &mut Game::new(9, 2);

    insta::assert_snapshot!(turn(game, Black, (0, 0), (0, 0), 1, CW));
    insta::assert_snapshot!(turn(game, White, (0, 0), (0, 0), 1, CW));
    insta::assert_snapshot!(turn(game, White, (0, 1), (0, 0), 5, CW));
    insta::assert_snapshot!(turn(game, Black, (0, 3), (0, 3), 2, CCW));
}

fn configure_insta() -> SettingsBindDropGuard {
    let mut settings = Settings::clone_current();
    settings.set_omit_expression(true);
    return settings.bind_to_scope();
}

fn turn(game: &mut Game, player: Player, (r, c): (usize, usize), (sr, sc): (usize, usize), ss: usize, dir: SpinDirection) -> String {
    let turn = Turn {
        player: player,
        play_row: r,
        play_col: c,
        spin_ul_row: sr,
        spin_ul_col: sc,
        spin_size: ss,
        spin_dir: dir,
    };

    let initial = game.to_string();
    let result  = game.turn(turn);
    let ending  = game.to_string();

    let mut ret = format!(
        "{:?} plays at ({}, {}) and spins from ({}, {}) {} tiles {:?}... => {result:?}\n",
        turn.player,
        turn.play_row,
        turn.play_col,
        turn.spin_ul_row,
        turn.spin_ul_col,
        turn.spin_size,
        turn.spin_dir
    );

    ret.push_str(&juxtapose(&initial, &ending));
    return ret;
}

fn juxtapose(a: &str, b: &str) -> String {
    let mut a_lines = a.lines();
    let mut b_lines = b.lines();
    let a_width = a.lines().fold(0, |best, next| best.max(len_colored(next)));

    let mut ret = "".to_string();
    let margin = 2;
    
    loop {
        match (a_lines.next(), b_lines.next()) {
            (Some(a_line), Some(b_line)) => {
                ret.push_str(a_line);
                ret.push_str(&" ".repeat(a_width - len_colored(a_line) + margin));
                ret.push_str(b_line);
            },
            (None, Some(b_line)) => {
                ret.push_str(&" ".repeat(a_width + margin));
                ret.push_str(b_line);
            },
            (Some(a_line), None) => {
                ret.push_str(a_line);
            },
            (None, None) => {
                break;
            },
        }

        ret.push_str("\n");
    }

    return ret;
}

// Visual length of an ANSI-colored string.

fn len_colored(s: &str) -> usize {
    let mut ret = 0;
    let mut counting = true;

    for c in s.to_owned().chars() {
        if c == '\x1b' {counting = false;}
        if counting {ret += 1;}
        if c == 'm' {counting = true;}
    }

    ret
}

