# Cake Addict

Based on [Rust in Action](https://www.manning.com/books/rust-in-action)

## short Description

A dungeon crawler with proc-gen levels, and fighting the monsters.

## Some differences

- Implementation fully in Bevy, does not use bracket-lib
  - Including its own implementation of Djikstra Maps
  - Heavily uses the Plugin idea for splitting up systems, probably not that well.
- Uses Turborand for making the randomness only once per game
- Uses ndarray for storing the map
- Aiming for using Mazes for programmers as a basis for the maps
- Includes a Quest system
