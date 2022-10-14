use super::{Puz, Size, Token};
use crate::color;
use crate::utils::ft_parse;
use crate::{err, err_no};
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
        while *i < f.len() && f[*i].trim().starts_with('#') && f[*i].trim().len() > 0 {
            *i += 1;
        }
        if *i == f.len() {
            err!("unexpected end of file");
        }
        return true;
    }

    fn _read_size(&mut self, line: &String) -> bool {
        let s = line.splitn(2, '#').next().unwrap().trim().to_string();
        self._size = match ft_parse::<Size>(&s) {
            Ok(n) => n,
            Err(_) => return false,
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
    // 1 2 3 # comment
    // 4  5 6
    // #another comment
    // 7 8 9
    fn _read_grid(&mut self, f: &Vec<String>, mut i: usize) -> bool {
        let mut n_extend: usize = 0;
        while i < f.len() && n_extend <= self._size as usize {
            let line = f[i].splitn(2, '#').next().unwrap().trim().to_string();

            if f[i].starts_with('#') || line.len() == 0 {
                i += 1;
                continue;
            }

            let nums = match self._parse_line(&line, i) {
                Ok(n) => n,
                Err(_) => return false,
            };

            if n_extend == self._size as usize {
                if nums.len() > 0 {
                    err!(
                        "expected no more value, got \"{R}{l}{C}{B}\"",
                        l = f[i],
                        C = color::CLEAR,
                        B = color::BOLD,
                        R = color::RED
                    );
                }
            } else {
                if nums.len() != self._size.into() {
                    err!("expected {G}{s}{C} {B}numbers on line {M}{i}{C}{B}, got {R}{n}{C}\n\t{B}\"{M}{line}{C}{B}\"",
					s = self._size, i = i + 1, n = nums.len(), line = f[i],
					C = color::CLEAR, B = color::BOLD, G = color::GRE, R = color::RED, M = color::MAG);
                }

                self._board.extend(nums);
                n_extend += 1;
            }

            i += 1;
        }

        if n_extend != self._size as usize {
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

    fn _parse_line(&self, line: &String, i: usize) -> Result<Vec<Token>, bool> {
        let mut nums: Vec<Token> = Vec::new();
        for n in line.split_whitespace() {
            match ft_parse::<Token>(n) {
                Ok(n) => match self._board.contains(&n) {
                    true => {
                        err_no!(
                            "duplicate {R}{n}{C}\t{B}{I}(second on line {C}{B}{M}{l}{C}{B})",
                            n = n,
                            l = i + 1,
                            C = color::CLEAR,
                            B = color::BOLD,
                            I = color::ITALIC,
                            R = color::RED,
                            M = color::MAG
                        );
                        return Err(false);
                    }
                    false => match n < (self._size as Token).pow(2) {
                        false => {
                            err_no!(
								"expected {G}0{C} {B}<= {I}n{C} {B}<= {G}{s}{C}{B}, got {R}{n}{C}\n\t{B}\"{M}{l}{C}{B}\"\t{I}(line {C}{B}{M}{i}{C}{B}{I})",
								n = n,
								s = (self._size as Token).pow(2) - 1,
								l = line,
								i = i + 1,
								C = color::CLEAR,
								B = color::BOLD,
								I = color::ITALIC,
								G = color::GRE,
								R = color::RED,
								M = color::MAG
							);
                            return Err(false);
                        }
                        true => nums.push(n),
                    },
                },
                Err(_) => return Err(false),
            }
        }
        return Ok(nums);
    }
}
