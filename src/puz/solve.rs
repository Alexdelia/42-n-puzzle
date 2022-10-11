use super::Puz;

impl Puz {
    pub fn solve(&mut self) {}

    fn get_manathan_distance(&self) -> u32 {
        let mut distance = 0;

        for i in 0..self._size.pow(2) {
            let mut x = 0;
            let mut y = 0;
            let mut target_x = 0;
            let mut target_y = 0;

            for f in 0..self._size.pow(2) {
                if self._board[f as usize] == i.into() {
                    x = f % self._size;
                    y = f / self._size;
                }
                if self._target[f as usize] == i.into() {
                    target_x = f % self._size;
                    target_y = f / self._size;
                }
            }
            distance += (x as i32 - target_x as i32).abs() as u32
                + (y as i32 - target_y as i32).abs() as u32;
        }
        return distance;
    }
}
