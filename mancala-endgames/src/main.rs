extern crate mancala_endgames;

use mancala_endgames::mancala::{Mancala,perfect_game};

fn main() {
    for n in 0..40 {
        let endgame: Mancala = perfect_game(n);

        println!("{:>3}: {}", n+1, endgame);
    }
}
