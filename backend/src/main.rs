
use kurakura::Game;
use std::time::{Instant, Duration};
use std::thread::sleep;
use std::collections::HashMap;
use uuid::Uuid;

fn main() {
    //let game = Game::new(2, 2);
    //println!("{game}\n");

    //let now = Instant::now();

    //sleep(Duration::new(2, 0));
    //println!("{}", now.elapsed().as_secs());


    //let mut items = HashMap::new();
    //items.insert("hello world", 3);
    //items.insert("goodbye world", 4);
    //items.insert("lalala", 6);

    //println!("{items:?}");
    //println!("{:?}", items.get("hello world"));
    //println!("{:?}", items.get("hell world"));

    let id = Uuid::new_v4();
    println!("{}", id.to_string());
}

