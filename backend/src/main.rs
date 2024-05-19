
#[allow(unused)] // TODO: disable this

mod game;
use crate::game::Twirl;
use crate::game::Player;
use crate::game::SpinDirection::*;
use crate::game::Player::*;

fn main() {
}

#[test]
fn test() {
    insta::with_settings!({
        omit_expression => true,
    }, {

        let mut game = Twirl::new(9, 2);
        insta::assert_snapshot!(play(&mut game, Black, 0, 0));
    });
}

fn play(game: &mut Twirl, player: Player, r: usize, c: usize) -> String {
    let initial = game.to_string();
    let result  = game.play(player, r, c);
    let ending  = game.to_string();

    let mut ret = format!("{player:?} plays at ({r}, {c})... => {result:?}\n");

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

