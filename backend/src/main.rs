
#[allow(unused)] // TODO: disable this

mod game;
use crate::game::Twirl;
use crate::game::Color::*;
use crate::game::Color;

/*
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
*/

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

fn play(game: &mut Twirl, color: Color, r: usize, c: usize) -> String {
    let initial = game.to_string();
    let result  = game.play(color, r, c);
    let ending  = game.to_string();

    let mut ret = format!("{color:?} plays at ({r}, {c})...  => {result:?}\n\n");

    ret.push_str(&juxtapose(&initial, &ending));
    return ret;
}

fn juxtapose(a: &str, b: &str) -> String {
    let mut a_lines = a.lines();
    let mut b_lines = b.lines();
    let a_width = a.lines().fold(0, |best, next| best.max(next.len()));

    let mut ret = "".to_string();
    let margin = 4;
    
    loop {
        match (a_lines.next(), b_lines.next()) {
            (Some(a_line), Some(b_line)) => {
                ret.push_str(a_line);
                ret.push_str(&" ".repeat(a_width - a_line.len() + margin));
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


