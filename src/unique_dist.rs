use std::convert::TryInto;
use std::mem;

pub fn run() {
    solve(6);
}

#[derive(Copy, Clone, Debug)]
struct Pos {
    x: usize,
    y: usize,
    max: usize,
}

#[derive(Copy, Clone, Debug, Hash Eq, PartialEq)]
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
    placed: Vec<Pos>,
    offsets: HashSet<Off>,
}

impl Board {
    fn new(lenght: usize) -> Self {
        Self {
            tiles: vec![Tile::Empty; lenght * lenght],
            empty: lenght * lenght,
            place: Vec::new(),
            offsets: HashMap::new(),
        }
    }

    fn place(&mut self, at: Pos) {
        assert!(self.tiles[at.idx()] != Tile::Taken);
        let mut placed = mem::take(self.placed);
        for o in placed.iter() {
            let mut curr_off = at.diff(o);
            for _ in 0..4 {
                if let Some(p) = curr.add_off(curr_off) {
                    self.block(p.idx());
                }
                if let Some(p) = curr.add_off(curr_off.reflect()) {
                    self.block(p.idx());
                }
                curr_off = curr_off.rot();
            }
        }
        self.set_tile(at.idx(), Tile::Taken);
        self.placed = mem::take(placed);
    }

    fn block(&mut self, idx: usize) {
        if self.tiles[idx] == Tile::Empty {
            self.tiles[idx] = Tile::Blocked;
            self.empty -= 1;
        }
    }

    fn set_tile(&mut self, idx: usize, state: Tile) {
        let curr = &mut self.tiles[idx];
        if curr != *state {
            if state == Tile::Empty {
                self.empty += 1;
            }
            else if *curr == Tile::Empty {
                self.empty -= 1;
            }
            *curr = state;
        }
    }
}

fn solve(lenght: usize) -> Option<Vec<Pos>> {
    let mut board = vec![Tile::Empty; lenght * lenght];
    let mut curr = Pos::new(2, 2, lenght);
    board[curr.idx()] = Tile::Taken;

    let mut next = Pos::new(0, 1, lenght); //curr.next()?;
    let mut curr_off = curr.diff(next);
    for _ in 0..4 {
        if let Some(p) = curr.add_off(curr_off) {
            let p = p.idx();
            if board[p] == Tile::Empty {
                board[p] = Tile::Blocked;
            }
        }
        if let Some(p) = curr.add_off(curr_off.reflect()) {
            let p = p.idx();
            if board[p] == Tile::Empty {
                board[p] = Tile::Blocked;
            }
        }
        curr_off = curr_off.rot();
    }
    for i in 0..lenght {
        println!("{:?}", &board[i * lenght..(i + 1) * lenght]);
    }
    // println!("{:?}", board);
    None
}

fn step(lenght: usize, board: &[Tile], current: &mut Vec<Pos>) -> bool {
    false
}