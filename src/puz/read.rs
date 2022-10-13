use super::{Puz, Size, Token};
use crate::color;
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
                }
                _ => {
                    err!("\"\x1b[35m{}\x1b[0m\x1b[1m\" {}", file, e);
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
        if !self._read_grid(&f, i + 1) {
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
        }
        return true;
    }

    fn _read_size(&mut self, line: &String) -> bool {
        self._size = match line.parse::<Size>() {
            Ok(s) => s,
            Err(e) => {
                err!(
                    "{e}\n\t\"{M}{line}{C}{B}\"",
                    e = e,
                    line = line,
                    C = color::CLEAR,
                    B = color::BOLD,
                    M = color::MAG
                );
            }
        };
        if self._size < 2 {
            err!(
                "expected size >= {G}2{C}{B}, got {R}{s}",
                s = self._size,
                C = color::CLEAR,
                B = color::BOLD,
                G = color::GRE,
                R = color::RED
            );
        }
        println!("size: {}", self._size);
        return true;
    }

    // need to read an initial state of format:
    // 1 2 3#comment
    // 4  5 6
    // #another comment
    // 7 8 9
    fn _read_grid(&mut self, f: &Vec<String>, mut i: usize) -> bool {
        let mut n_extend: usize = 0;
        while i < f.len() && n_extend < self._size as usize {
            let line = f[i].clone();
            if line.starts_with('#') {
                i += 1;
                continue;
            }

            let s = line.splitn(2, '#').next().unwrap().to_string();

            let mut nums: Vec<Token> = Vec::new();
            for n in s.split_whitespace() {
                nums.push(match n.parse::<Token>() {
                    Ok(n) => n,
                    Err(e) => {
                        err!(
                            "{e}\n\t\"{M}{line}{C}{B}\"",
                            e = e,
                            line = line,
                            C = color::CLEAR,
                            B = color::BOLD,
                            M = color::MAG
                        );
                    }
                });
            }

            if nums.len() != self._size.into() {
                err!("expected {G}{s}{C} {B}numbers on line {M}{i}{C}{B}, got {R}{n}{C}\n\t{B}\"{M}{line}{C}{B}\"",
					s=self._size, i=i, n=nums.len(), line=line,
					C=color::CLEAR, B=color::BOLD, G=color::GRE, R=color::RED, M=color::MAG);
            }

            self._board.extend(nums);
            n_extend += 1;

            i += 1;
        }

        if n_extend != (self._size - 1) as usize {
            err!(
                "expected {G}{s}{C} {B}lines, got {R}{n}",
                s = self._size,
                n = n_extend,
                C = color::CLEAR,
                B = color::BOLD,
                G = color::GRE,
                R = color::RED
            );
        }

        return true;
    }
}
