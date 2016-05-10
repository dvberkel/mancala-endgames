use std::fmt;

struct Mancala {
    pub position: Vec<u16>,
}

fn perfect_position(n: u8) -> Mancala {
    Mancala { position: vec![1] }
}

impl fmt::Display for Mancala {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[").unwrap();
        for beans in &self.position {
            write!(f, " {}", beans).unwrap();
        }
        write!(f, " ]")
    }
}

fn main() {
    let endgame = perfect_position(1);

    println!("{}", endgame);
}
