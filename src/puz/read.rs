use super::Puz;
use crate::err;
use std::fs::File;
use std::io::ErrorKind;

impl Puz {
    pub fn read(&mut self, file: String) -> bool {
        let f = match File::open(&file) {
            Ok(f) => f,
            Err(e) => match e.kind() {
                ErrorKind::NotFound => {
                    // will need to decide of that or format!()
                    err!("\x1b[35m", file, "\x1b[0m\x1b[1m not found");
                    return false;
                }
                _ => {
                    println!("Error opening file");
                    return false;
                }
            },
        };

        return true;
    }
}
