use crate::puz::r#move::Move;
use crate::puz::{Size, Token};

pub fn get_target_snail(size: Size) -> Vec<Token> {
    let size: Token = size as Token;
    let mut target: Vec<Token> = vec![0; (size as usize).pow(2)];
    let mut x: Token = 0;
    let mut y: Token = 0;
    let mut dir: Move = Move::Right;
    let mut i: Token = 1;

    while i < size.pow(2) {
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
    crate::puz::Puz::print_other(&target, size as Size);
    println!();

    return target;
}

pub fn get_target_classic(size: Size) -> Vec<Token> {
    let mut target: Vec<Token> = (1..(size as Token).pow(2)).collect();
    target.push(0);

    // debug
    println!("Target:");
    crate::puz::Puz::print_other(&target, size);
    println!();

    return target;
}
