use crate::block::Direction;
use crate::game::Game;
use crate::mcts::MonteCarloStrategy;
use crate::pathfinder::PathFindingStrategy;
use crate::random::RandomStrategy;
use std::ops::Deref;

pub trait Strategy {
    fn tick(&self, game: &Game) -> Direction;
}

pub enum StrategyImpl {
    Random(RandomStrategy),
    PathFinder(PathFindingStrategy),
    MonteCarlo(MonteCarloStrategy),
}

impl Deref for StrategyImpl {
    type Target = dyn Strategy;

    fn deref(&self) -> &Self::Target {
        match self {
            StrategyImpl::Random(s) => s,
            StrategyImpl::PathFinder(s) => s,
            StrategyImpl::MonteCarlo(s) => s,
        }
    }
}
