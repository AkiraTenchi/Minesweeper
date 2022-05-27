use rand::{thread_rng, Rng};
use std::{collections::HashSet, hash::Hash};

pub type Position = (usize, usize);
pub enum OpenResult {
    Mine,
    NoMine(u8),
}

#[derive(Debug)]
pub struct Minesweeper {
    width: usize,
    height: usize,
    open_fields: HashSet<Position>,
    mines: HashSet<Position>,
    flagged_fields: HashSet<Position>,
}

impl Minesweeper {
    pub fn new(width: usize, height: usize, mine_count: usize) -> Minesweeper {
        Minesweeper {
            width,
            height,
            open_fields: HashSet::new(),
            mines: Minesweeper::gen_mines(width, height, mine_count),
            flagged_fields: HashSet::new(),
        }
    }

    pub fn open(&mut self, position: Position) -> OpenResult {
        self.open_fields.insert(position);

        let is_mine = self.mines.contains(&position);

        if is_mine {
            OpenResult::Mine
        } else {
            OpenResult::NoMine(0)
        }
    }

    fn neighbours(&self, (x, y): Position) -> impl Iterator<Item = Position> {
        let width = self.width;
        let height = self.height;

        (x.max(1) - 1..=(x + 1).min(width - 1))
            .flat_map(move |i| (y.max(1) - 1..=(y + 1).min(height - 1)).map(move |j| (i, j)))
            .filter(move |&pos| pos != (x, y))
    }

    fn neighbouring_mines(&self, (x, y): Position))

    fn gen_mines(width: usize, height: usize, mine_count: usize) -> HashSet<Position> {
        let mut rng = thread_rng();
        let mut mines = HashSet::new();
        while mines.len() < mine_count {
            mines.insert((rng.gen_range(0..width), rng.gen_range(0..height)));
        }
        mines
    }
}

#[cfg(test)]
mod tests {
    use crate::Minesweeper;

    #[test]
    fn test() {
        let ms = Minesweeper::new(10, 10, 5);

        println!("{:?}", ms);
    }
}
