use crate::color;

pub fn usage() {
    println!("{B}usage:   {G}./{file}{C} {B}{CY}<heuristic>{C} {B}{BL}<strategy>{C} {B}{M}<initial_state>{C} {B}{M}{D}<goal_state>{C} {B}{R}{D}<stop_at_first_solution>{C}",
		file = std::env::args().next().unwrap(),
		C = color::CLEAR,
		B = color::BOLD,
		D = color::DIM,
		R = color::RED,
		G = color::GRE,
		BL = color::BLU,
		M = color::MAG,
		CY = color::CYA,
	);
    println!();
    usage_heuristic();
    println!();
    usage_strategy();
    println!();
    usage_initial_state();
    println!();
    usage_goal_state();
    println!();
    usage_stop_at_first_solution();
    println!();
    println!("{B}default: {G}./{file}{C} {B}{CY}manhattan{C} {B}{BL}astar{C} {B}{M}3{C} {B}{M}{D}snail{C} {B}{R}{D}true{C}",
        file = std::env::args().next().unwrap(),
        C = color::CLEAR,
        B = color::BOLD,
        D = color::DIM,
		R = color::RED,
        G = color::GRE,
        BL = color::BLU,
        M = color::MAG,
        CY = color::CYA,
	);
    println!();
}

pub fn usage_heuristic() {
    println!(
		"	{B}{CY}<heuristic>:{C} {B}{M}manhattan{C} {B}| {M}hamming{C} {B}| {M}linear_conflict{C} {B}| {M}euclidean{C}
		{I}{B}({Y}optional{C} {I}| {M}default: {B}manhattan{C} {I}| {Y}first letter is enough{C}{I}{B}){C}",
		C = color::CLEAR,
		B = color::BOLD,
		I = color::ITALIC,
		Y = color::YEL,
		M = color::MAG,
		CY = color::CYA,
	);
}

pub fn usage_strategy() {
    println!(
        "	{B}{BL}<strategy>:{C} {B}{M}astar{C} {B}| {M}greedy{C} {B}| {M}uniform{C}
		{I}{B}({Y}optional{C} {I}| {M}default: {B}astar{C}     {I}| {Y}first letter is enough{C}{I}{B}){C}",
        C = color::CLEAR,
        B = color::BOLD,
        I = color::ITALIC,
        Y = color::YEL,
        M = color::MAG,
        BL = color::BLU,
    );
}

pub fn usage_initial_state() {
    println!(
		"	{B}{M}<initial_state>:{C} {B}{M}{U}file{C} {B}| {M}{U}number{C}{B}:{C} {I}(random n-puzzle of size {M}{B}{U}n{C}{I}){C}
		{I}{B}({Y}optional{C} {I}| {M}default: {B}3{C}){C}",
		C = color::CLEAR,
		B = color::BOLD,
		I = color::ITALIC,
		U = color::UNDERLINE,
		Y = color::YEL,
		M = color::MAG,
	);
}

pub fn usage_goal_state() {
    println!(
        "	{B}{M}{D}<goal_state>:{C} {B}{M}{U}file{C} {B}| {M}snail{C} {B}| {M}classic{C}
		{I}{B}({Y}optional{C} {I}| {M}default: {B}snail{C}     {I}| {Y}first letter is enough{C}{I}{B}){C}",
        C = color::CLEAR,
        B = color::BOLD,
        I = color::ITALIC,
        D = color::DIM,
        U = color::UNDERLINE,
        Y = color::YEL,
        M = color::MAG,
    );
}

pub fn usage_stop_at_first_solution() {
    println!(
        "	{B}{R}{D}<stop_at_first_solution>:{C} {B}{M}true{C} {B}| {M}false{C}
		{I}{B}({Y}optional{C} {I}| {M}default: {B}true{C}      {I}| {Y}first letter is enough{C}{I}{B}){C}",
        C = color::CLEAR,
        B = color::BOLD,
        I = color::ITALIC,
        D = color::DIM,
        Y = color::YEL,
        M = color::MAG,
        R = color::RED,
    );
}
