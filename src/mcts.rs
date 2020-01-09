use crate::block::Direction;
use crate::consts::{GRID_HEIGHT, GRID_WIDTH};
use crate::game::Game;
use crate::strategy::Strategy;
use fnv::FnvHashMap;
use rand::{thread_rng, Rng};
use rayon::prelude::*;

const DEFAULT_RUNS: usize = 10_000;

#[derive(Debug)]
struct Run {
    start: Direction,
    score: u32,
}

fn run(game: &Game) -> Run {
    let mut game = game.clone();
    let start = loop {
        let s: Direction = thread_rng().gen();
        if s != game.snake.direction.opposite() {
            break s;
        }
    };
    let mut run = Run { start, score: 0 };
    let mut dir = run.start;
    let mut moves_without_eating = 0;
    while moves_without_eating < 10 * GRID_WIDTH * GRID_HEIGHT * GRID_WIDTH * GRID_HEIGHT {
        game.snake.turn(dir);
        run.score += 1;
        if game.scores() {
            run.score += 50;
        } else if game.collides() {
            break;
        } else {
            moves_without_eating += 1;
        }
        game.update();
        dir = thread_rng().gen();
    }
    run
}

pub struct MonteCarloStrategy;

impl Strategy for MonteCarloStrategy {
    fn tick(&self, game: &Game) -> Direction {
        (0..DEFAULT_RUNS)
            .into_par_iter()
            .map(|_| run(game))
            .fold(
                FnvHashMap::<Direction, (f32, usize)>::default,
                |mut acc, run| {
                    let (avg, n) = acc.entry(run.start).or_default();
                    *avg = (*avg * (*n as f32) + run.score as f32) / (*n + 1) as f32;
                    *n += 1;
                    acc
                },
            )
            .reduce(FnvHashMap::default, |mut acc, map| {
                for (k, (avg, n)) in map {
                    let (avg_acc, n_acc) = acc.entry(k).or_default();
                    *avg_acc = (*avg_acc * *n_acc as f32 + avg * n as f32) / (*n_acc + n) as f32;
                    *n_acc += n;
                }
                acc
            })
            .iter()
            .max_by_key(|(_, (avg, _))| *avg as u32)
            .map(|(k, _)| *k)
            .unwrap()
    }
}
