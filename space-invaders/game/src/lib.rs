mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
  fn alert(s: &str);
}

#[wasm_bindgen]
#[derive(Copy, Clone)]
pub struct Vector {
  pub x: f64,
  pub y: f64,
}

#[wasm_bindgen]
impl Vector {
  #[wasm_bindgen(constructor)]
  pub fn new(x: f64, y: f64) -> Vector {
    Vector { x, y }
  }

  pub fn subtract(&self, other: &Vector) -> Vector {
    Vector::new(self.x - other.x, self.y - other.y)
  }

  pub fn scale_by(&self, number: f64) -> Vector {
    Vector::new(self.x * number, self.y * number)
  }
}

pub struct Alien {}

pub struct Cannon {}

#[wasm_bindgen]
pub struct Game {
  pub width: i32,
  pub height: i32,
  pub speed: f64,
  pub score: i32,
  pub lives: u8,
  pub cannon_position: Vector,
}

#[wasm_bindgen]
impl Game {
  pub fn new(width: i32, height: i32, speed: f64) -> Game {
    Game {
      width: width,
      height: height,
      speed: speed,
      score: 0,
      lives: 3,
      cannon_position: Vector::new(0.0, 0.0),
    }
  }

  pub fn tick(&self) {}
}
