use crate::consts::{BLOCK_SIZE, GRID_HEIGHT, GRID_WIDTH};
use piston_window::types::Color;
use piston_window::{rectangle, Context, Graphics};
use rand::distributions::{Distribution, Standard};
use rand::Rng;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    pub fn all() -> &'static [Direction; 4] {
        static DIRS: [Direction; 4] = [
            Direction::Left,
            Direction::Right,
            Direction::Up,
            Direction::Down,
        ];
        &DIRS
    }

    pub fn opposite(self) -> Direction {
        use Direction::*;
        match self {
            Left => Right,
            Right => Left,
            Up => Down,
            Down => Up,
        }
    }
}

impl Distribution<Direction> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Direction {
        match rng.gen_range(0, 4) {
            0 => Direction::Left,
            1 => Direction::Right,
            2 => Direction::Up,
            3 => Direction::Down,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Block {
    pub x: u32,
    pub y: u32,
}

impl Block {
    pub fn new(x: u32, y: u32) -> Block {
        Block { x, y }
    }

    pub fn draw<G: Graphics>(self, color: Color, ctx: &Context, gfx: &mut G) {
        rectangle(
            color,
            [
                (self.x * BLOCK_SIZE + 1) as f64,
                (self.y * BLOCK_SIZE + 1) as f64,
                (BLOCK_SIZE - 2) as f64,
                (BLOCK_SIZE - 2) as f64,
            ],
            ctx.transform,
            gfx,
        );
    }

    pub fn apply(self, dir: Direction) -> Option<Block> {
        let mut block = self;
        match dir {
            Direction::Left if block.x > 0 => block.x -= 1,
            Direction::Right if block.x < GRID_WIDTH - 1 => block.x += 1,
            Direction::Up if block.y > 0 => block.y -= 1,
            Direction::Down if block.y < GRID_HEIGHT - 1 => block.y += 1,
            _ => return None,
        }
        Some(block)
    }

    pub fn manhattan_distance(self, other: Block) -> u32 {
        ((self.x as i32 - other.x as i32) + (self.y as i32 - other.y as i32)).abs() as u32
    }
}
