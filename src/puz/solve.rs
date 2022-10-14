use super::Puz;

use super::r#move::{AllowedMove, Move};
use super::{Size, Token};

impl Puz {
    pub fn solve(&mut self) {
        let mut m = AllowedMove::new_array();
        let mut dist = (self._manathan_distance(), Move::Up);

        while dist.0 > 0 {
            self.update_allowed_move(&mut m);
            dist = self._best_move(&m);
            self.play_move(dist.1);
            self.print();
        }
    }

    fn _best_move(&mut self, m: &[AllowedMove; 4]) -> (u32, Move) {
        let mut best = (u32::MAX, Move::Up);
        for i in 0..4 {
            print!("({}", i);
            if m[i].a {
                self.play_move(m[i].m);
                let dist = self._manathan_distance();
                if dist < best.0 {
                    best = (dist, m[i].m);
                }
                print!(": {}", dist);
                self.play_move(Move::get_opposite(m[i].m));
            }
            print!(")\t");
        }
        println!();
        return best;
    }

    fn _manathan_distance(&self) -> u32 {
        let size = self._size as Token;
        let mut distance = 0;

        for i in 0..size.pow(2) {
            let mut x = 0;
            let mut y = 0;
            let mut target_x = 0;
            let mut target_y = 0;

            for f in 0..size.pow(2) {
                if self._board[f as usize] == i.into() {
                    x = f % size;
                    y = f / size;
                }
                if self._target[f as usize] == i.into() {
                    target_x = f % size;
                    target_y = f / size;
                }
            }
            distance += (x as i32 - target_x as i32).abs() as u32
                + (y as i32 - target_y as i32).abs() as u32;
        }
        return distance;
    }

    pub fn is_solvable(&self) -> bool {
        let size = self._size as Token;
        let mut inversions = 0;
        let mut blank_row = 0;

        for x in 0..size.pow(2) {
            if self._board[x as usize] == 0 {
                blank_row = x / size;
                continue;
            }
            for y in x..size.pow(2) {
                if self._board[y as usize] == 0 {
                    continue;
                }
                if self._board[x as usize] > self._board[y as usize] {
                    inversions += 1;
                }
            }
        }

        if size % 2 == 1 {
            return inversions % 2 == 1;
        } else {
            return (inversions + blank_row) % 2 == 0;
        }
    }
}
