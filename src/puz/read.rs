use super::Puz;
use crate::err;
use std::fs::File;
use std::io::{BufRead, BufReader, ErrorKind};

impl Puz {
    pub fn read(&mut self, file: &String) -> bool {
        let f = match File::open(file) {
            Ok(f) => f,
            Err(e) => match e.kind() {
                ErrorKind::NotFound => {
                    err!("\"\x1b[35m{}\x1b[0m\x1b[1m\" not found", file);
                    return false;
                }
                _ => {
                    err!("\"\x1b[35m{}\x1b[0m\x1b[1m\" {}", file, e);
                    return false;
                }
            },
        };

        let f = BufReader::new(f);
        let f: Vec<String> = f.lines().map(|l| l.unwrap()).collect();
        let mut i: usize = 0;

        if !Self::_read_skip(&f, &mut i) {
			return false;
		}
        if !self._read_size(&f[i]) {
            return false;
        }

        return true;
    }

    fn _read_skip(f: &Vec<String>, i: &mut usize) -> bool {
        while *i < f.len() && f[*i].starts_with('#') && f[*i].len() > 0 {
            *i += 1;
        }
		if *i == f.len() {
			err!("unexpected end of file");
			return false;
		}
		return true;
    }

    fn _read_size(&mut self, line: &String) -> bool {
        self._size = match line.parse::<super::Size>() {
            Ok(s) => s,
            Err(e) => {
                err!("{}\n\t\x1b[1m{}\x1b[0m", e, line);
                return false;
            }
        };
        println!("size: {}", self._size);
        return true;
    }

	fn _read_initial_state(&mut self, f: &Vec<String>, i: &mut usize) 
}
