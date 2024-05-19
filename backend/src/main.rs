
mod game;
mod test;

use crate::game::Twirl;

fn main() {
    let game = Twirl::new(9, 2);
    println!("{game}");
}

