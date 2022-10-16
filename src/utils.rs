use crate::color;

#[macro_export]
macro_rules! err_no {
	($($arg:tt)*) => {
		eprint!("\x1b[31;1merror\x1b[0m\x1b[1m:\t");
		eprint!($($arg)*);
		eprintln!("\x1b[0m");
	};
}

#[macro_export]
macro_rules! err {
	($($arg:tt)*) => {
		err_no!($($arg)*);
		return false;
	};
}

// str to int with type of int in param
pub fn ft_parse<F: std::str::FromStr>(s: &str) -> Result<F, bool>
where
    <F as std::str::FromStr>::Err: std::fmt::Display,
{
    match s.parse::<F>() {
        Ok(n) => Ok(n),
        Err(e) => {
            err_no!("{e}\n\t\"{M}{s}{C}{B}\" to type {G}{type}",
        	    e = e,
        	    s = s,
        	    type = std::any::type_name::<F>(),
        	    C = color::CLEAR,
        	    B = color::BOLD,
        	    M = color::MAG,
        	    G = color::GRE);
            Err(false)
        }
    }
}
