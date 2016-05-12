extern crate mancala_endgames;

use std::env;
use mancala_endgames::mancala::{Mancala,perfect_game};

fn main() {
    let mut arguments = env::args();
    match arguments.nth(1) {
        Some(argument) => {
            let n: i32 = argument.parse()
                .ok()
                .expect("I need a number");
            let endgame: Mancala = perfect_game(n-1);

            println!("{}", endgame);
        },
        None => { println!("Supply an numeric argument"); }
    }
}
