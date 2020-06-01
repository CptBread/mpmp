use std::convert::TryInto;
use std::mem;
use std::collections::HashSet;
use std::io::{self, Read};

pub fn run() {
    solve(6);
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Pos {
    x: usize,
    y: usize,
    max: usize,
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
struct Off {
    x: isize,
    y: isize,
}

impl Pos {
    fn new(x: usize, y: usize, max: usize) -> Self {
        Self {
            x, y, max,
        }
    }

    fn next(&self) -> Option<Self> {
        let x = self.x + 1;
        if x >= self.max {
            let y = self.y + 1;
            if y >= self.max {
                None
            }
            else {
                Some(Pos::new(0, y, self.max))
            }
        }
        else{
            Some(Pos::new(x, self.y, self.max))
        }
    }

    fn diff(&self, o: Self) -> Off {
        Off{x:self.x as isize - o.x as isize, y: self.y as isize - o.y as isize}
    }

    fn dist2(&self, o: Self) -> f32 {
        (self.x as f32 - o.x as f32).powf(2.0) + (self.y as f32 - o.y as f32).powf(2.0)
    }

    fn dist(&self, o: Self) -> f32 {
        self.dist2(o).sqrt()
    }

    fn idx(&self) -> usize {
        self.x + self.y * self.max
    }

    fn from_idx(idx: usize, lenght: usize) -> Self {
        Pos::new(idx % lenght, idx / lenght, lenght)
    }

    fn add_off(&self, off: Off) -> Option<Self> {
        let x = if let Ok(x) = (self.x as isize + off.x).try_into() {x} else {return None;};
        let y = if let Ok(y) = (self.y as isize + off.y).try_into() {y} else {return None;};
        if x >= self.max || y >= self.max {
            None
        } 
        else{
            Some(Self {
                x,
                y,
                max: self.max,
            })
        }
    }
}

impl Off {
    fn rot(&self) -> Self {
        Off{x: self.y, y: -self.x}
    }

    fn reflect(&self) -> Self {
        Off{x: self.x, y: -self.y}
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Tile{
    Empty,
    Taken,
    Blocked,
}

#[derive(Clone, Debug)]
struct Board {
    tiles: Vec<Tile>,
    empty: usize,
    lenght: usize,
    placed: Vec<Pos>,
    offsets: HashSet<Off>,
}

impl Board {
    fn new(lenght: usize) -> Self {
        Self {
            tiles: vec![Tile::Empty; lenght * lenght],
            empty: lenght * lenght,
            lenght,
            placed: Vec::new(),
            offsets: HashSet::new(),
        }
    }

    fn place(&mut self, at: Pos) -> bool {
        assert!(self.tiles[at.idx()] != Tile::Taken);
        let mut offsets = mem::take(&mut self.offsets);
        for off in offsets.iter() {
            self.block_off(at, *off);
        }
        let mut placed = mem::take(&mut self.placed);
        for p in placed.iter() {
            let mut off = at.diff(*p);
            if !offsets.contains(&off) {
                let mut fail = 0;
                for _ in 0..4 {
                    let reflect = off.reflect();
                    if offsets.insert(off) {
                        for o in placed.iter() {
                            self.block_off(*o, off);
                        }
                        if !self.block_off(at, off) {
                            fail += 1;
                        }
                    } 
                    if offsets.insert(reflect) {
                        for o in placed.iter().chain(std::iter::once(&at)) {
                            self.block_off(*o, reflect);
                        }
                        if !self.block_off(at, reflect) {
                            fail += 1;
                        }
                    }
                    off = off.rot();
                }
                if fail > 1 {
                    return false;
                }
            }
        }
        self.set_tile(at.idx(), Tile::Taken);
        placed.push(at);
        self.placed = placed;
        self.offsets = offsets;
        true
    }

    fn first_empty(&self) -> Option<Pos> {
        if self.empty <= 0 {
            return None;
        }
        for (idx, t) in self.tiles.iter().enumerate() {
            if *t == Tile::Empty {
                return Some(Pos::from_idx(idx, self.lenght));
            }
        }
        panic!("Shouldn't be possible to get here!");
    }

    fn next_empty(&self, at: Pos) -> Option<Pos> {
        if self.empty <= 0 {
            return None;
        }
        for (idx, t) in self.tiles.iter().enumerate().skip(at.idx() + 1) {
            if *t == Tile::Empty {
                return Some(Pos::from_idx(idx, self.lenght));
            }
        }
        return None;
    }

    // False if already taken
    fn block(&mut self, idx: usize) -> bool {
        match self.tiles[idx] {
            Tile::Empty => {
            self.tiles[idx] = Tile::Blocked;
            self.empty -= 1;
                true
            },
            Tile::Blocked => {true}
            Tile::Taken => {false}
        }
    }

    // False if already taken
    fn block_off(&mut self, at: Pos, off: Off) -> bool {
        if let Some(p) = at.add_off(off) {
            self.block(p.idx())
        }
        else {
            true
        }
    }

    fn set_tile(&mut self, idx: usize, state: Tile) {
        let curr = &mut self.tiles[idx];
        if *curr != state {
            if state == Tile::Empty {
                self.empty += 1;
            }
            else if *curr == Tile::Empty {
                self.empty -= 1;
            }
            *curr = state;
        }
    }

    fn show(&self) {
        for i in 0..self.lenght {
            let line = format!("{:?}", &self.tiles[i * self.lenght..(i + 1) * self.lenght])
                .replace("Empty", "_").replace("Taken", "X").replace("Blocked", "0");
            println!("{}", line);
        }
    }

    fn check_dist(&self) -> bool {
        let max = self.placed.len();
        let mut dists = HashSet::new();
        for f in 0..max {
            for s in (f + 1)..max {
                let f = self.placed[f];
                let s = self.placed[s];
                println!("{} {:?} {:?}, {:?}", f.dist(s), f, s, f.diff(s));
                if !dists.insert(f.dist(s).to_ne_bytes()) {
                    return false;
                }
            }
        }
        return true;
    }
}

fn solve(lenght: usize) -> Option<Vec<Pos>> {
    // let mut board = vec![Tile::Empty; lenght * lenght];
    // let mut curr = Pos::new(2, 2, lenght);
    // board[curr.idx()] = Tile::Taken;

    // let mut next = Pos::new(0, 1, lenght); //curr.next()?;
    // let mut curr_off = curr.diff(next);
    // for _ in 0..4 {
    //     if let Some(p) = curr.add_off(curr_off) {
    //         let p = p.idx();
    //         if board[p] == Tile::Empty {
    //             board[p] = Tile::Blocked;
    //         }
    //     }
    //     if let Some(p) = curr.add_off(curr_off.reflect()) {
    //         let p = p.idx();
    //         if board[p] == Tile::Empty {
    //             board[p] = Tile::Blocked;
    //         }
    //     }
    //     curr_off = curr_off.rot();
    // }
    let mut board = Board::new(lenght);
    step(&mut board);
    // board.place(Pos::new(2, 2, lenght));
    // board.place(Pos::new(0, 1, lenght));
    // loop {
    //     board.show();
    //     if let Some(p) = board.first_empty() {
    //         board.place(p);
    //         println!("Place: ({}, {})", p.x, p.y);
    //     }
    //     else {
    //         println!("Failed with {} placed", board.placed.len());
    //         break;
    //     }
    // }
    println!("{:?}", board);
    None
}

fn step(last: &mut Board) -> bool {
    let mut at = if let Some(p) = last.first_empty() {p} else {return false};
    let mut board = last.clone();
    loop {
        if !board.place(at) {
            last.block(at.idx());
            last.show();
            println!("Failed placed at: {:?}", at);
            at = if let Some(p) = last.next_empty(at) {p} else {return false};
            board = last.clone();
            continue;
        }
        if board.placed.len() == board.lenght {
            board.show();
            println!("Found solution: {:?}", board.placed);
            if board.check_dist() {
                return true;
            }
            // return read_int().is_none()
        }
        if board.empty == 0 {
            board.show();
            println!("Failed with: {:?}", board.placed);
            return false;
        }
        if step(&mut board) {
            return true;
        }
        board = last.clone();
        at = if let Some(p) = last.next_empty(at) {p} else {return false};
    }
}

// fn step(last: &Board) -> bool {
//     last.show();
//     let at = if let Some(p) = read_pos(last.lenght) {p} else {return false};
//     let mut board = last.clone();
//     board.place(at);
//     if board.placed.len() == board.lenght {
//         // board.show();
//         println!("Found solution: {:?}", board.placed);
//         return true
//     }
//     if board.empty == 0 {
//         board.show();
//         println!("Failed with: {:?}", board.placed);
//         return false;
//     }
//     if step(&board) {
//         return true;
//     }
//     step(last)
// }

// fn read_pos(len: usize) -> Option<Pos> {
//     Some(Pos::new(read_int()?, read_int()?, len))
// }

fn read_int() -> Option<usize> {
    let mut n = String::new();
    io::stdin().read_line(&mut n).ok()?;
    n.trim().parse().ok()
}