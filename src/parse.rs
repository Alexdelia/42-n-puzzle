use crate::color;
use crate::err_no;
use crate::puz::heuristic;
use crate::puz::target_type;
use crate::puz::{Puz, Size, Strategy, Token};
use crate::usage;
use crate::utils::ft_parse;
use std::env::args;
use std::fs::File;

pub fn parse() -> Result<Puz, bool> {
    let mut p: Puz;
    let av = args().collect::<Vec<String>>();

    if av.len() > 3 {
        p = match parse_size(av[3].as_str()) {
            Ok(p) => p,
            Err(_) => return Err(false),
        };
    } else {
        p = Puz::from(3);
    }

    match parse_heuristic(av[1].as_str()) {
        Ok(h) => p.set_heuristic(h),
        Err(_) => return Err(false),
    }

    if av.len() > 2 {
        match parse_strategy(av[2].as_str()) {
            Ok(s) => p.set_strategy(s),
            Err(_) => return Err(false),
        }
    }

    if av.len() > 4 {
        if !parse_goal_state(av[4].as_str(), &mut p) {
            return Err(false);
        }
    } else {
        p.set_target(&target_type::get_target_snail(p.get_size()));
    }

    if av.len() > 5 {
        match parse_stop_at_first_solution(av[5].as_str()) {
            Ok(b) => p.set_stop_at_first_solution(b),
            Err(_) => return Err(false),
        };
    }

    return Ok(p);
}

fn parse_size(a: &str) -> Result<Puz, bool> {
    let mut p: Puz;

    if File::open(a).is_ok() {
        p = Puz::new();
        if !p.read(a, false) {
            return Err(false);
        }
    } else {
        let n = ft_parse::<Size>(a);
        if n.is_ok() {
            if n.unwrap() < 2 {
                err_no!(
                    "expected size >= {G}2{C}{B}, got {R}{s}",
                    s = a,
                    C = color::CLEAR,
                    B = color::BOLD,
                    R = color::RED,
                    G = color::GRE
                );
                return Err(false);
            }
            p = Puz::from(n.unwrap());
        } else {
            Puz::new().read(a, false); // recall to get error message
            err_no!(
                "\"{M}{a}{C}{B}\" is not a valid size or file",
                a = a,
                C = color::CLEAR,
                B = color::BOLD,
                M = color::MAG
            );
            usage::usage_initial_state();
            return Err(false);
        }
    };

    return Ok(p);
}

fn parse_heuristic(a: &str) -> Result<fn(&[Token], Size, &[Token]) -> u32, bool> {
    match a {
        "m" | "manhattan" => Ok(heuristic::manathan_distance),
        _ => {
            err_no!(
                "\"{M}{a}{C}{B}\" is not a valid heuristic",
                a = a,
                C = color::CLEAR,
                B = color::BOLD,
                M = color::MAG
            );
            usage::usage_heuristic();
            return Err(false);
        }
    }
}

fn parse_strategy(a: &str) -> Result<Strategy, bool> {
    match a {
        "a" | "astar" => Ok(Strategy::AStar),
        "g" | "greedy" => Ok(Strategy::Greedy),
        "u" | "uniform" => Ok(Strategy::Uniform),
        _ => {
            err_no!(
                "\"{M}{a}{C}{B}\" is not a valid strategy",
                a = a,
                C = color::CLEAR,
                B = color::BOLD,
                M = color::MAG
            );
            usage::usage_strategy();
            return Err(false);
        }
    }
}

fn parse_goal_state(a: &str, p: &mut Puz) -> bool {
    match a {
        "s" | "snail" => p.set_target(&target_type::get_target_snail(p.get_size())),
        "c" | "classic" => p.set_target(&target_type::get_target_classic(p.get_size())),
        _ => {
            if !p.read(a, true) {
                err_no!(
                    "\"{M}{a}{C}{B}\" is not a valid goal_state",
                    a = a,
                    C = color::CLEAR,
                    B = color::BOLD,
                    M = color::MAG
                );
                usage::usage_goal_state();
                false;
            }
        }
    }
    return true;
}

fn parse_stop_at_first_solution(a: &str) -> Result<bool, bool> {
    match a {
        "t" | "true" => Ok(true),
        "f" | "false" => Ok(false),
        _ => {
            err_no!(
                "\"{M}{a}{C}{B}\" is not a valid stop_at_first_solution",
                a = a,
                C = color::CLEAR,
                B = color::BOLD,
                M = color::MAG
            );
            usage::usage_stop_at_first_solution();
            return Err(false);
        }
    }
}
