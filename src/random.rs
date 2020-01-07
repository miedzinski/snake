use crate::block::Direction;
use crate::game::Game;
use crate::strategy::Strategy;
use rand::{thread_rng, Rng};

pub struct RandomStrategy;

impl Strategy for RandomStrategy {
    fn tick(&self, _game: &Game) -> Direction {
        thread_rng().gen()
    }
}
