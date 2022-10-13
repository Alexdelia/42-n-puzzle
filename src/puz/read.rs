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

        let r = BufReader::new(f);

        for l in r.lines() {
            let line = l.unwrap();
        }

        return true;
    }
}
