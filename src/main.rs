use std::io;
use std::env;

#[derive(Copy, Clone)]
struct TriPos {
    pub line: u8,
    pub pos: u8,
    pub max_line: u8,
}

impl TriPos {
    pub fn new(line: u8, pos: u8, max_line: u8) -> Self {
        Self {
            line, pos, max_line,
        }
    }

    pub fn to_off(&self) -> u8 {
        line_to_offset(self.line) + self.pos
    }

    pub fn to_idx(&self) -> usize {
        self.to_off() as usize
    }

    fn move_ul(&self, l_off: u8, p_off: u8) -> Option<Self> {
        let res = Self {
            line: self.line.checked_sub(l_off)?,
            pos: self.pos.checked_sub(p_off)?,
            max_line: self.max_line,
        };
        if res.pos > res.line {
            None
        }
        else {
            Some(res)
        }
    }

    fn move_dr(&self, l_off: u8, p_off: u8) -> Option<Self> {
        let line = self.line + l_off;
        let pos = self.pos + p_off;
        if line >= self.max_line || pos > line {
            None
        }
        else 
        {
            Some(Self {
                line, pos,
                max_line: self.max_line,
            })
        }
    }

    pub fn up_left(&self) -> Option<Self> {
        self.move_ul(1, 1)
    }

    pub fn up_right(&self) -> Option<Self> {
        self.move_ul(1, 0)
    }

    pub fn left(&self) -> Option<Self> {
        self.move_ul(0, 1)
    }

    pub fn right(&self) -> Option<Self> {
        self.move_dr(0, 1)
    }

    pub fn down_left(&self) -> Option<Self> {
        self.move_dr(1, 0)
    }

    pub fn down_right(&self) -> Option<Self> {
        self.move_dr(1, 1)
    }
}

fn line_to_offset(line: u8) -> u8 {
    if line == 0 {
        0
    }
    else {
        ((line - 1) * line) / 2 + line
    }
}

#[derive(Clone, Debug)]
struct Moves {
    score: u8,
    moves: Vec<(u8, u8)>,
}

impl Moves {
    fn new() -> Self {
        Self {
            score: 0,
            moves: Vec::new(),
        }
    }

    // Formats the moves into the puzzle format. Bad becuase anything not using 0 index is shameful. Or I guess not primarily meant for programmers... But isn't that the same thing?
    fn to_bad_str(&self) -> String {
        let mut res = format!("Score: {}\nMoves:", self.score);
        let mut last = (0, 0);
        let mut first = true;
        for pair in &self.moves {
            if pair.0 == std::u8::MAX {
            }
            else if pair.0 == last.1 {
                res.push_str(&format!("-{}", pair.1 + 1));
            }
            else {
                if first {
                    first = false;
                    res.push_str(&format!(" {}-{}", pair.0 + 1, pair.1 + 1));
                }
                else {
                    res.push_str(&format!(", {}-{}", pair.0 + 1, pair.1 + 1));
                }
            }
            last = *pair;
        }
        res
    }
}

#[derive(Copy, Clone, Debug)]
struct MoveCheck {
    len: u8,
    moves: [(u8, u8); 6],
}

fn main() {
    let sides = env::args().skip(1).next().and_then(|s| s.parse::<u8>().ok()).unwrap_or(4);
    solve_tri_solitair(sides);
}

fn solve_tri_solitair(side: u8) {
    let num = line_to_offset(side) as usize;
    let mut pegs = vec![true; num];
    let mut moves_at_idx = vec![MoveCheck{len: 0, moves: [(0u8, 0u8); 6]}; num];
    let mut idx = 0;
    // cache possible moves for each cell
    for l in 0..side {
        for n in 0..=l {
            let pos = TriPos::new(l, n, side);
            let mut try_add = |func: fn(&TriPos) -> Option<TriPos>, pos| {
                if let Some(p0) = func(&pos) {
                    if let Some(p1) = func(&p0) {
                        let m = &mut moves_at_idx[idx];
                        m.moves[m.len as usize] = (p0.to_off(), p1.to_off());
                        m.len += 1;
                    }
                }
            };
            try_add(TriPos::up_left, pos);
            try_add(TriPos::up_right, pos);
            try_add(TriPos::right, pos);
            try_add(TriPos::left, pos);
            try_add(TriPos::down_left, pos);
            try_add(TriPos::down_right, pos);
            idx += 1;
        }
    }

    let start_moves = if side == 4 {
        vec![0, 1, 4]
    }
    else if side == 5 {
        vec![0, 1, 3, 4]
    }
    else if side == 6 {
        vec![0, 1, 3, 4, 2]
    }
    else if side < 4 {
        Vec::new() // They will all fail anyways...
    }
    else {
        panic!("I didn't create a starting move calculator because of lazyness... So can only do ones hard coded right now... Also anything bigger whould just take to long anyways...")
    };

    let mut moves = Moves::new();
    
    let mut best = if side == 6 {
        // We assume we can find a path better then this as otherwise it's just going to take too long...
        Some(Moves{
            score: 10,
            moves: Vec::new(),
        })
    }
    else {
        None
    };

    let moves_left = pegs.len() - 1;
    for s in &start_moves {
        let s = *s;
        pegs[s] = false;
        moves.moves.push((std::u8::MAX, s as u8));
        solve_tri_solitair_rec(moves_left as u8, &mut pegs, &moves_at_idx, &mut moves, &mut best);
        moves.moves.pop();
        pegs[s] = true;
        eprint!(".");
    }
    eprint!("\n");

    println!("{:?}", best);
    if let Some(res) = best {
        println!("\n{}", res.to_bad_str());
    }
}

fn solve_tri_solitair_rec(left: u8, pegs: &mut [bool], move_at_idx: &Vec<MoveCheck>, current: &mut Moves, best: &mut Option<Moves>) {
    if left == 1 {
        if best.as_ref().map(|b| b.score).unwrap_or(std::u8::MAX) > current.score {
            *best = Some(current.clone());
        }
        return;
    }
    // If best is better or equal then the current score skip
    if best.as_ref().map(|b| b.score <= current.score).unwrap_or(false) {
        return;
    }

    for idx in 0..pegs.len() {
        if pegs[idx] {
            let m = &move_at_idx[idx];
            for (mid, target) in &m.moves[0..m.len as usize] {
                let mid = *mid as usize;
                let target = *target as usize;
                if pegs[mid] && !pegs[target] {
                    let mut cost = 1;
                    if let Some(last) = current.moves.last() {
                        if last.1 == idx as u8 {
                            cost = 0;
                        }
                    }

                    pegs[idx] = false;
                    pegs[mid] = false;
                    pegs[target] = true;
                    current.score += cost;
                    current.moves.push((idx as u8, target as u8));
                    solve_tri_solitair_rec(left - 1, pegs, &move_at_idx, current, best);
                    current.moves.pop();
                    current.score -= cost;
                    pegs[idx] = true;
                    pegs[mid] = true;
                    pegs[target] = false;
                }
            }
        }
    }
}

fn test_walk(start: TriPos) {
    let mut pos = start;
    let mut input = String::new();
    loop {
        for l in 0..pos.max_line {
            print!("\n{}\t", l + 1);
            for _ in 0..(pos.max_line - l) {
                print!(" ");
            }
            for n in 0..=l {
                if pos.line == l && pos.pos == n {
                    print!("* ");
                }
                else {
                    print!("O ");
                }
            }
        }
        print!("\n");

        input.clear();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let slice = input.trim();
                match slice {
                    "ul" | "7" => pos = pos.up_left().unwrap_or(pos),
                    "ur" | "9" => pos = pos.up_right().unwrap_or(pos),
                    "l" | "4" => pos = pos.left().unwrap_or(pos),
                    "r" | "6" => pos = pos.right().unwrap_or(pos),
                    "dl" | "1" => pos = pos.down_left().unwrap_or(pos),
                    "dr" | "3" => pos = pos.down_right().unwrap_or(pos),
                    "exit" => break,
                    _ => print!("Bad input"),
                }
            }
            Err(error) => panic!("error: {}", error),
        }
    }
}