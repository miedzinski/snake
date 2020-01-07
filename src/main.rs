#[macro_use]
extern crate clap;
extern crate fnv;
extern crate piston_window;
extern crate rand;
extern crate rayon;

mod block;
mod consts;
mod game;
mod mcts;
mod pathfinder;
mod random;
mod strategy;

use crate::consts::{BG_COLOR, BLOCK_SIZE, GRID_HEIGHT, GRID_WIDTH, TICK_RATE};
use crate::mcts::MonteCarloStrategy;
use crate::pathfinder::PathFindingStrategy;
use crate::random::RandomStrategy;
use crate::strategy::StrategyImpl;
use clap::{app_from_crate, SubCommand};
use piston_window::{
    clear, Button, EventLoop, PistonWindow, PressEvent, UpdateEvent, WindowSettings,
};

fn main() {
    let matches = app_from_crate!()
        .subcommand(SubCommand::with_name("random").about("Random moves"))
        .subcommand(SubCommand::with_name("pathfinder").about("Shortest path to food"))
        .subcommand(SubCommand::with_name("monte-carlo").about("Monte Carlo Tree Search"))
        .get_matches();

    let playing = matches.subcommand_name().is_none();
    let strategy: StrategyImpl = match matches.subcommand_name().unwrap_or("") {
        "random" => StrategyImpl::Random(RandomStrategy),
        "pathfinder" => StrategyImpl::PathFinder(PathFindingStrategy),
        "monte-carlo" => StrategyImpl::MonteCarlo(MonteCarloStrategy),
        _ => StrategyImpl::Random(RandomStrategy),
    };

    let mut window: PistonWindow =
        WindowSettings::new("Snake", [GRID_WIDTH * BLOCK_SIZE, GRID_HEIGHT * BLOCK_SIZE])
            .exit_on_esc(true)
            .resizable(false)
            .build()
            .unwrap();
    window.set_ups(TICK_RATE);
    window.set_max_fps(TICK_RATE);

    let mut game = game::Game::new();

    while let Some(event) = window.next() {
        window.draw_2d(&event, |ctx, gfx, _| {
            clear(BG_COLOR, gfx);
            game.draw(&ctx, gfx);
        });
        if playing {
            event.press(|btn: Button| {
                if let Button::Keyboard(key) = btn {
                    game.key(key);
                }
            });
        } else if game.running {
            event.update(|_| {
                game.snake.turn(strategy.tick(&game));
            });
        }
        event.update(|_| game.update());
    }
}
