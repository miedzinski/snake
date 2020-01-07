use crate::block::{Block, Direction};
use crate::consts::{FOOD_COLOR, GRID_HEIGHT, GRID_WIDTH, HEAD_COLOR, START_SIZE, TAIL_COLOR};
use piston_window::keyboard::Key;
use piston_window::{Context, Graphics};
use rand::{thread_rng, Rng};
use std::collections::VecDeque;

#[derive(Clone)]
pub struct Snake {
    pub blocks: VecDeque<Block>,
    pub direction: Direction,
    pub last_growth_direction: Direction,
}

impl Snake {
    pub fn new() -> Snake {
        Snake {
            blocks: (0..START_SIZE).rev().map(|x| Block::new(x, 0)).collect(),
            direction: Direction::Right,
            last_growth_direction: Direction::Right,
        }
    }

    pub fn draw<G: Graphics>(&self, ctx: &Context, gfx: &mut G) {
        let mut iter = self.blocks.iter();
        iter.next().unwrap().draw(HEAD_COLOR, ctx, gfx);
        for b in iter {
            b.draw(TAIL_COLOR, ctx, gfx);
        }
    }

    pub fn head(&self) -> Block {
        *self.blocks.front().unwrap()
    }

    pub fn last(&self) -> Block {
        *self.blocks.back().unwrap()
    }

    pub fn turn(&mut self, dir: Direction) {
        if dir != self.last_growth_direction.opposite() {
            self.direction = dir;
        }
    }

    fn grow(&mut self) {
        self.blocks
            .push_front(self.head().apply(self.direction).unwrap());
        self.last_growth_direction = self.direction;
    }

    fn shrink(&mut self) {
        self.blocks.pop_back();
    }
}

#[derive(Clone, Copy)]
pub struct Food {
    pub block: Block,
}

impl Food {
    pub fn random() -> Food {
        let x = thread_rng().gen_range(0, GRID_WIDTH);
        let y = thread_rng().gen_range(0, GRID_HEIGHT);
        Food {
            block: Block::new(x, y),
        }
    }

    pub fn draw<G: Graphics>(self, ctx: &Context, gfx: &mut G) {
        self.block.draw(FOOD_COLOR, ctx, gfx);
    }
}

#[derive(Clone)]
pub struct Game {
    pub snake: Snake,
    pub food: Food,
    pub running: bool,
}

impl Game {
    pub fn new() -> Game {
        let mut game = Game {
            snake: Snake::new(),
            food: Food {
                block: Block::new(0, 0),
            },
            running: true,
        };
        game.init();
        game
    }

    pub fn draw<G: Graphics>(&self, ctx: &Context, gfx: &mut G) {
        self.food.draw(ctx, gfx);
        self.snake.draw(ctx, gfx);
    }

    pub fn update(&mut self) {
        if !self.running {
            return;
        }
        if self.collides() {
            self.running = false;
            return;
        }
        if self.scores() {
            self.spawn_food();
        } else {
            self.snake.shrink();
        }
        self.snake.grow();
    }

    pub fn key(&mut self, key: Key) {
        match key {
            Key::Space => self.running = !self.running,
            Key::R => self.init(),
            Key::Left => self.snake.turn(Direction::Left),
            Key::Right => self.snake.turn(Direction::Right),
            Key::Up => self.snake.turn(Direction::Up),
            Key::Down => self.snake.turn(Direction::Down),
            _ => (),
        }
    }

    pub fn init(&mut self) {
        self.snake = Snake::new();
        self.spawn_food();
        self.running = true;
    }

    pub fn spawn_food(&mut self) {
        self.food = loop {
            let f = Food::random();
            if !(self.snake.blocks.contains(&f.block)
                || self.snake.head().apply(self.snake.direction) == Some(f.block))
            {
                break f;
            }
        }
    }

    pub fn scores(&self) -> bool {
        self.snake.head().apply(self.snake.direction) == Some(self.food.block)
    }

    pub fn collides(&self) -> bool {
        let head = self.snake.head();
        let next = head.apply(self.snake.direction);
        let mut iter = self.snake.blocks.iter().skip(1);
        iter.next_back(); // Skip last block so it's possible to follow tail.
        if iter.any(|&x| next == Some(x)) {
            return true;
        }
        match self.snake.direction {
            Direction::Left if head.x == 0 => true,
            Direction::Right if head.x == GRID_WIDTH - 1 => true,
            Direction::Up if head.y == 0 => true,
            Direction::Down if head.y == GRID_HEIGHT - 1 => true,
            _ => false,
        }
    }
}
