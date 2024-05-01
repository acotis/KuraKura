
mod game;
use crate::game::Twirl;

fn main() {
    let game = Twirl::new(9);
    println!("{game}");
}
