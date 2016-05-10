use std::fmt;

struct Mancala {
    pub position: Vec<i32>,
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
    let endgame = Mancala { position: vec![1, 2, 3, 4] };

    println!("{}", endgame);
}
