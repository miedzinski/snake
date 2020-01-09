# Snake

Classic snake game & AI playing it.

![](snake.gif?raw=true)

# Installation

You can install directly from git repository with

    cargo install --git https://github.com/miedzinski/snake.git

This will install binary `snake` to `$CARGO_HOME/bin/` (`~/.cargo/bin/` by default).

# Usage

Running `snake` without arguments will start the game.
Press arrows to move, space to pause/start, R to restart, and ESC to exit.

`snake --strategy <strategy>` will start specified algorithm.
Options currently available:

- `random`
- `pathfinder`
- `monte-carlo`

`snake --help` displays help.

# Algorithms

## Random

Issues random move every tick.

## Pathfinder

Searches for shortest path from head to food and follows if it's
possible to reach snake's tail after eating, otherwise follows tail.
If there is no path from head to tail snake moves randomly.

## Monte Carlo

A variant of MCTS called Pure Monte Carlo tree search, in which selection step
and all further moves in simulation are chosen randomly.

# License

MIT.

Copyright (c) 2020 Dominik Miedziński.
