use super::{Size, Token};

pub fn manathan_distance(board: &[Token], size: Size, target: &[Token]) -> u32 {
    let size: Token = size as Token;
    let mut distance = 0;

    for i in 0..size.pow(2) {
        let mut x = 0;
        let mut y = 0;
        let mut target_x = 0;
        let mut target_y = 0;

        for f in 0..size.pow(2) {
            if board[f as usize] == i.into() {
                x = f % size;
                y = f / size;
            }
            if target[f as usize] == i.into() {
                target_x = f % size;
                target_y = f / size;
            }
        }
        distance +=
            (x as i32 - target_x as i32).abs() as u32 + (y as i32 - target_y as i32).abs() as u32;
    }
    return distance;
}