use crate::puz::r#move::Move;
use crate::puz::{Size, Token};

pub fn get_target_snake(size: Size) -> Vec<Token> {
    let mut target: Vec<Token> = vec![0; size.pow(2) as usize];
    let mut x: Size = 0;
    let mut y: Size = 0;
    let mut i: Token = 1;
    let mut dir: Move = Move::Right;

    while i < size.pow(2).into() {
        target[(x * size + y) as usize] = i;
        i += 1;
        match dir {
            Move::Right => {
                if y == size - 1 || target[(x * size + y + 1) as usize] != 0 {
                    dir = Move::Down;
                    x += 1;
                } else {
                    y += 1;
                }
            }
            Move::Down => {
                if x == size - 1 || target[((x + 1) * size + y) as usize] != 0 {
                    dir = Move::Left;
                    y -= 1;
                } else {
                    x += 1;
                }
            }
            Move::Left => {
                if y == 0 || target[(x * size + y - 1) as usize] != 0 {
                    dir = Move::Up;
                    x -= 1;
                } else {
                    y -= 1;
                }
            }
            Move::Up => {
                if x == 0 || target[((x - 1) * size + y) as usize] != 0 {
                    dir = Move::Right;
                    y += 1;
                } else {
                    x -= 1;
                }
            }
        }
    }

    // debug
    println!("Target:");
    crate::puz::Puz::print_other(&target, size);
    println!();

    return target;
}