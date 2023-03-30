/*
	An implementation of the Sidewinder algorithm
		https://weblog.jamisbuck.org/2011/2/3/maze-generation-sidewinder-algorithm
		For each row in the grid:
		1. For each cell randomly decide whether to carve a passage leading East
			a. If the passage is carved add the cell to the current run set
			b. If the passage is not carved, randomly pick one cell from the route
				set, carve a passage leading North and empty the current run set

*/

use wasm_bindgen::prelude::*;
use core::ops::Range;
use js_sys::Math;
use enumflags2::{bitflags, make_bitflags, BitFlags};
use crate::log;


#[wasm_bindgen]
#[bitflags]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wall {
	North,
	South,
	East,
	West,
}

pub fn random(range: Range<i32>) -> i32 {
	let start = range.start as f64;
	let end = range.end as f64;
	Math::floor(Math::random() * f64::from((end - start + 1.0) + start)) as i32
}

pub struct Maze {
	pub width: usize,
	pub height: usize,
	pub grid: Vec<BitFlags<Wall>>,
}

impl Maze {
	pub fn new(width: usize, height: usize) -> Maze {
		Maze {
			width,
			height,
			grid: vec![BitFlags::all(); width * height],
		}
	}

	pub fn build(&mut self) {
		let mut run_start = 0;
		for y in 0..self.height {
			run_start = 0;
			for x in 0..self.width {
				if y > 0 && (x + 1 == self.width || Math::random() <= 0.5) {
					let end = random(0..(x - run_start + 1) as i32) as usize;
					let cell = run_start + end;
					self.remove_wall(y, cell, Wall::North);
					self.remove_wall(y - 1, cell, Wall::South);
					run_start = x + 1
				} else if (x + 1 < self.width) {
					self.remove_wall(y, x, Wall::East);
					self.remove_wall(y, x + 1, Wall::West);
				}
			}
		}
	}


	fn remove_wall(&mut self, x: usize, y: usize, wall: Wall) {
		// log!("Trying to set index ({}, {}) ({})", x + offset, y, self.index(x + offset, y));
		let index = self.index(x, y);
		self.grid[index] &= !wall;
	}

	fn index(&self, x: usize, y: usize) -> usize {
		y * self.width + x
	}

	pub fn as_ptr(&self) -> *const BitFlags<Wall> {
		self.grid.as_ptr()
	}
}
