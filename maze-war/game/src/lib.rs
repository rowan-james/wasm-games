mod utils;
mod maze;

extern crate js_sys;
extern crate web_sys;

use wasm_bindgen::prelude::*;
use maze::{Maze, Wall};
use enumflags2::BitFlags;

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

pub(crate) use log;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Point {
    pub x: i32,
    pub y: i32
}

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Player {
    pub position: Point,
    pub score: i32
}

#[wasm_bindgen]
pub struct Game {
    players: Vec<Player>,
    maze: Maze
}

#[wasm_bindgen]
impl Game {
    #[wasm_bindgen(constructor)]
    pub fn new(width: usize, height: usize) -> Game {
        utils::set_panic_hook();
        log!("Creating game with size of ({}, {})", width, height);
        Game {
            players: vec![],
            maze: Maze::new(width, height)
        }
    }
    
    pub fn start(&mut self) {
        self.maze.build();
    }

    pub fn player_count(&self) -> usize {
        self.players.iter().count()
    }

    pub fn maze_as_ptr(&self) -> *const BitFlags<Wall> {
        self.maze.as_ptr()
    }
}
