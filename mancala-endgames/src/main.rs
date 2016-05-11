use std::fmt;

pub struct Mancala {
    pub position: Vec<i32>,
}

pub fn perfect_position(n: i32) -> Mancala {
    if n == 0 {
        Mancala { position: vec![1] }
    } else {
        let mut position: Vec<i32> = perfect_position(n - 1).position.to_vec();
        position.push(n + 1);
        decrement_below(n, &mut position);
        fill_zeros_below(n, &mut position);
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

pub fn fill_zeros_below(n: i32, position: &mut Vec<i32>) {
    let mut index = 0;
    while index < n {
        if position[index as usize] == 0 {
            fill_zero_at(index, position);
        }
        index += 1;
    }
}

pub fn fill_zero_at(index: i32, position: &mut Vec<i32>) {
    if position[index as usize] != 0 { panic!("position at index is not zero"); }
    if index == 0 {
        let mut prefix_index = 0;
        while position[prefix_index as usize] == 0 {
            prefix_index += 1;
        }
        prefix_index -= 1;
        let prefix: Vec<i32> = perfect_position(prefix_index).position.to_vec();
        while prefix_index >= 0 {
            let i = prefix_index as usize;
            position[i] = prefix[i];
            prefix_index -= 1;
        }
    } else {
        position[index as usize] = index + 1;
        decrement_below(index, position);
        fill_zeros_below(index, position);
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

    #[test]
    fn fill_zero_at_should_work_correctly_with_zero_not_in_first_place() {
        let mut position = vec![2, 0, 3];

        fill_zero_at(1, &mut position);

        assert_eq!(vec![1, 2, 3], position);
    }

   #[test]
    fn fill_zero_at_should_work_correctly_with_zero_in_first_place() {
        let mut position = vec![0, 1, 3];

        fill_zero_at(0, &mut position);

        assert_eq!(vec![1, 1, 3], position);
    }

    #[test]
    fn perfect_position_should_work_correctly() {
        assert_eq!(vec![1], perfect_position(0).position.to_vec());

        assert_eq!(vec![1, 2], perfect_position(1).position.to_vec());

        assert_eq!(vec![1, 1, 3], perfect_position(2).position.to_vec());

        assert_eq!(vec![1, 2, 2, 4], perfect_position(3).position.to_vec());
   }
}
