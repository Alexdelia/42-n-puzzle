use crate::color;

#[macro_export]
macro_rules! err {
	($($arg:tt)*) => {
		eprint!("\x1b[31;1merror\x1b[0m\x1b[1m:\t");
		eprint!($($arg)*);
		eprintln!("\x1b[0m");
		return false;
	};
}

// str to int with type of int in param
fn ft_parse<T>(s: &str, t: T) -> Result<T, bool> {
    match s.parse::<T>() {
        Ok(n) => n,
        Err(e) => {
            err!(
                "{e}\n\t\"{M}{s}{C}{B}\" to type {CY}{type}",
                e = e,
                s = s,
                type = T,
                C = color::CLEAR,
                B = color::BOLD,
                M = color::MAG,
                CY = color::CYA,
            );
        }
    }
}
