
#[allow(unused)] // TODO: disable this

mod game;
use crate::game::Twirl;
use crate::game::Player;
use crate::game::SpinDirection::*;
use crate::game::Player::*;

fn main() {
    let mut game = Twirl::new(9, 2);
    game.play(Black, 0, 0);
    game.spin(Black, 0, 0, 1, CW).unwrap();
    game.play(White, 0, 3);
    game.spin(White, 0, 0, 1, CW);
    println!("{}", game);
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

    let mut ret = format!("{player:?} plays at ({r}, {c})... => {result:?}\n\n");

    //ret.push_str(&juxtapose(&initial, &ending));
    return ret;
}


