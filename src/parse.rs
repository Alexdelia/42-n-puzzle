use crate::color;
use crate::err_no;
use crate::puz::heuristic;
use crate::puz::target_type;
use crate::puz::{Puz, Size, Strategy};
use crate::usage;
use crate::utils::ft_parse;
use std::env::args;
use std::fs::File;

pub fn parse() -> Result<Puz, bool> {
    let mut p: Puz;
    let av = args().collect::<Vec<String>>();

    if av.len() > 3 {
        p = match parse_av3(av[3].as_str()) {
            Ok(p) => p,
            Err(_) => return Err(false),
        };
    } else {
        p = Puz::from(3);
    }

    match av[1].as_str() {
        "m" | "manhattan" => p.set_heuristic(heuristic::manathan_distance),
        _ => {
            err_no!(
                "\"{M}{a}{C}{B}\" is not a valid heuristic",
                a = av[1],
                C = color::CLEAR,
                B = color::BOLD,
                M = color::MAG
            );
            usage::usage_heuristic();
            return Err(false);
        }
    };
    if av.len() > 2 {
        match av[2].as_str() {
            "a" | "astar" => p.set_strategy(Strategy::AStar),
            "g" | "greedy" => p.set_strategy(Strategy::Greedy),
            "u" | "uniform" => p.set_strategy(Strategy::Uniform),
            _ => {
                err_no!(
                    "\"{M}{a}{C}{B}\" is not a valid strategy",
                    a = av[2],
                    C = color::CLEAR,
                    B = color::BOLD,
                    M = color::MAG
                );
                usage::usage_strategy();
                return Err(false);
            }
        };
    } else {
        p.set_strategy(Strategy::AStar);
    }

    if av.len() > 4 {
        match av[4].as_str() {
            "s" | "snail" => p.set_target(&target_type::get_target_snail(p.get_size())),
            "c" | "classic" => p.set_target(&target_type::get_target_classic(p.get_size())),
            _ => {
                if !p.read(av[4].as_str(), true) {
                    err_no!(
                        "\"{M}{a}{C}{B}\" is not a valid goal_state",
                        a = av[4],
                        C = color::CLEAR,
                        B = color::BOLD,
                        M = color::MAG
                    );
                    usage::usage_goal_state();
                    return Err(false);
                }
            }
        };
    }

    return Ok(p);
}

fn parse_av3(a: &str) -> Result<Puz, bool> {
    let mut p: Puz;

    if File::open(a).is_ok() {
        p = Puz::new();
        if !p.read(a, false) {
            return Err(false);
        }
    } else {
        let n = ft_parse::<Size>(a);
        if n.is_ok() {
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
            usage();
            return Err(false);
        }
    };

    return Ok(p);
}
