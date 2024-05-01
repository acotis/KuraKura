
#[allow(unused)] // TODO: disable this

mod game;
use crate::game::Twirl;

fn main() {
    let game = Twirl::new(9);
    println!("{game}");
}
