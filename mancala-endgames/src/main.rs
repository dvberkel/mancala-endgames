use std::fmt;

struct Mancala {
    pub position: Vec<i32>,
}

fn perfect_position(n: i32) -> Mancala {
    if n == 0 {
        Mancala { position: vec![1] }
    } else {
        let mut position: Vec<i32> = perfect_position(n - 1).position.to_vec();
        position.push(n + 1);
        decrement_below(n, &mut position);
        Mancala { position: position }
    }
}

pub fn decrement_below(n: i32, position: &mut Vec<i32>) {
    let mut index = n;
    while index > 0  {
        index -= 1;
        position[index as usize] -= 1;
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decrement_below_should_work_correctly() {
        let mut position = vec![1, 2, 3, 4];

        decrement_below(2, &mut position);

        assert_eq!(vec![0, 1, 3, 4], position);
    }

    #[test]
    fn decrement_below_should_work_correctly_with_zero() {
         let mut position = vec![1, 2, 3, 4];

        decrement_below(0, &mut position);

        assert_eq!(vec![1, 2, 3, 4], position);
    }
}
