use crate::block::{Block, Direction};
use crate::game::Game;
use crate::strategy::Strategy;
use fnv::FnvHashMap;
use rand::{thread_rng, Rng};
use std::cmp::Ordering;
use std::collections::BinaryHeap;

struct Visit {
    current: Block,
    cost: u32,
    goal: Block,
}

impl Visit {
    fn estimated_cost(&self) -> u32 {
        self.cost + self.current.manhattan_distance(self.goal)
    }
}

impl PartialEq for Visit {
    fn eq(&self, other: &Visit) -> bool {
        self.estimated_cost() == other.estimated_cost()
    }
}

impl Eq for Visit {}

impl PartialOrd for Visit {
    fn partial_cmp(&self, other: &Visit) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Visit {
    fn cmp(&self, other: &Self) -> Ordering {
        self.estimated_cost().cmp(&other.estimated_cost()).reverse()
    }
}

#[derive(Clone, Copy)]
struct Step {
    cost: u32,
    direction: Option<Direction>,
}

impl Default for Step {
    fn default() -> Self {
        Step {
            cost: std::u32::MAX,
            direction: None,
        }
    }
}

pub struct PathFindingStrategy;

impl Strategy for PathFindingStrategy {
    fn tick(&self, game: &Game) -> Direction {
        self.find_safe_path_to_food(game)
            .or_else(|| self.find_path(game, game.snake.head(), game.snake.last()))
            .and_then(|x| x.first().copied())
            .unwrap_or_else(|| thread_rng().gen())
    }
}

impl PathFindingStrategy {
    fn find_path(&self, game: &Game, start: Block, goal: Block) -> Option<Vec<Direction>> {
        let mut to_visit = BinaryHeap::new();
        let mut costs: FnvHashMap<Block, Step> = FnvHashMap::default();

        to_visit.push(Visit {
            current: start,
            cost: 0,
            goal,
        });

        while let Some(visit) = to_visit.pop() {
            if visit.current == goal {
                let mut path = vec![];
                let mut block = visit.current;
                let mut step = costs.get(&block).copied().unwrap();
                while block != start {
                    let dir = step.direction.unwrap();
                    path.push(dir.opposite());
                    block = block.apply(dir).unwrap();
                    step = costs.get(&block).copied().unwrap_or_default();
                }
                path.reverse();
                return Some(path);
            }

            // Treat last block as unoccupied - it's legal to move there.
            let mut tail = game.snake.blocks.iter();
            tail.next_back();

            let iter = Direction::all()
                .iter()
                .map(|&x| (visit.current.apply(x), x.opposite()))
                .filter_map(|(block, dir)| match block {
                    Some(block) if !tail.clone().any(|&x| x == block) => Some((block, dir)),
                    _ => None,
                });

            for (neighbour, from) in iter {
                let next = Visit {
                    current: neighbour,
                    cost: visit.cost + 1,
                    goal,
                };
                let best = costs.entry(neighbour).or_default();
                if next.cost < best.cost {
                    best.cost = next.cost;
                    best.direction = Some(from);
                    to_visit.push(next);
                }
            }
        }

        None
    }

    fn find_safe_path_to_food(&self, game: &Game) -> Option<Vec<Direction>> {
        if let Some(path) = self.find_path(game, game.snake.head(), game.food.block) {
            let mut game_sim = game.clone();
            for &step in &path {
                game_sim.snake.turn(step);
                game_sim.update();
            }
            self.find_path(&game_sim, game_sim.snake.head(), game_sim.snake.last())
                .and(Some(path))
        } else {
            None
        }
    }
}
