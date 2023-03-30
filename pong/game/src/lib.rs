mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

use std::fmt;
extern crate js_sys;
extern crate web_sys;
use js_sys::Math;

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[wasm_bindgen]
#[derive(Copy, Clone, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Point {
        Point { x, y }
    }

    pub fn add(&self, other: &Point) -> Point {
        Point::new(self.x + other.x, self.y + other.y)
    }

    pub fn add_f64(&self, scalar: f64) -> Point {
        Point::new(self.x + scalar, self.y + scalar)
    }

    pub fn add_vector(&self, vector: Vector) -> Point {
        Point::new(self.x + vector.x, self.y + vector.y)
    }

    pub fn sub(&self, other: &Point) -> Point {
        Point::new(self.x - other.x, self.y - other.y)
    }

    pub fn sub_f64(&self, scalar: f64) -> Point {
        Point::new(self.x - scalar, self.y - scalar)
    }

    pub fn sub_vector(&self, vector: &Vector) -> Point {
        Point::new(self.x - vector.x, self.y - vector.y)
    }
}

#[wasm_bindgen]
#[derive(Copy, Clone, PartialEq)]
pub struct Rect {
    pub position: Point,
    pub width: f64,
    pub height: f64,
}

impl Rect {
    pub fn new(x: f64, y: f64, width: f64, height: f64) -> Rect {
        Rect {
            position: Point::new(x, y),
            width,
            height,
        }
    }

    pub fn min(&self) -> Point {
        self.position
    }

    pub fn max(&self) -> Point {
        Point::new(self.position.x + self.width, self.position.y + self.height)
    }

    pub fn contains_point(&self, other: &Point) -> bool {
        let a1 = self.min();
        let a2 = self.max();

        a1.x <= other.x && a2.x > other.x && a1.y <= other.y && a2.y > other.y
    }

    pub fn contains_rect(&self, other: &Rect) -> bool {
        let a1 = self.min();
        let a2 = self.max();
        let b1 = other.min();
        let b2 = other.max();

        a1.x >= b1.x && a2.x <= b2.x && a1.y >= b1.y && a2.y <= b2.y
    }

    pub fn overlaps(&self, other: &Rect) -> bool {
        let a1 = self.min();
        let a2 = self.max();
        let b1 = other.min();
        let b2 = other.max();

        a1.x < b2.x && a2.x > b1.x && a1.y < b2.y && a2.y > b1.y
    }
}

#[wasm_bindgen]
#[derive(Copy, Clone, PartialEq)]
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

    pub fn add(&self, other: &Vector) -> Vector {
        Vector::new(self.x + other.x, self.y + other.y)
    }

    pub fn subtract(&self, other: &Vector) -> Vector {
        Vector::new(self.x - other.x, self.y - other.y)
    }

    pub fn divide(&self, number: f64) -> Vector {
        Vector::new(self.x / number, self.y / number)
    }

    pub fn scale_by(&self, number: f64) -> Vector {
        Vector::new(self.x * number, self.y * number)
    }

    pub fn length(&self) -> f64 {
        self.x.hypot(self.y)
    }

    pub fn normalize(&self) -> Vector {
        self.scale_by(1_f64 / self.length())
    }

    pub fn from(angle: f64) -> Vector {
        Vector::new(Math::cos(angle), Math::sin(angle))
    }
}

impl fmt::Display for Vector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

trait Bounds {
    fn top_left(&self) -> Vector;
    fn bottom_right(&self) -> Vector;

    fn overlaps(&self, other: &dyn Bounds) -> bool {
        let a1 = self.top_left();
        let a2 = self.bottom_right();
        let b1 = other.top_left();
        let b2 = other.bottom_right();

        a1.x < b2.x && a2.x > b1.x && a1.y < b2.y && a2.y > b1.y
    }
}

#[wasm_bindgen]
#[derive(Debug)]
pub enum Movement {
    Up = 0,
    Down = 1,
}

impl fmt::Display for Movement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[wasm_bindgen]
#[derive(Copy, Clone)]
pub struct Paddle {
    pub bounds: Rect,
    pub speed: f64,
    pub score: i32,
}

impl Paddle {
    pub fn new(x: f64, y: f64, width: f64, height: f64, speed: f64) -> Paddle {
        Paddle {
            bounds: Rect::new(x, y, width, height),
            speed: speed,
            score: 0,
        }
    }

    pub fn increment_score(&mut self) {
        self.score += 1;
    }

    pub fn apply_movement(&mut self, delta_time: f64, movement: Vector) {
        let distance = self.speed * delta_time;
        let vector = movement.scale_by(distance);
        self.bounds.position = self.bounds.position.add_vector(vector);
    }

    pub fn process_movement(&mut self, direction: Movement) -> Vector {
        match direction {
            Movement::Up => Vector::new(0.0, -1.0),
            Movement::Down => Vector::new(0.0, 1.0),
        }
    }
}

#[wasm_bindgen]
#[derive(Copy, Clone)]
pub struct Ball {
    pub bounds: Rect,
    pub direction: Vector,
    pub speed: f64,
}

impl Ball {
    pub fn new(width: f64, height: f64, screen_width: f64, screen_height: f64, speed: f64) -> Ball {
        let starting_point = Ball::random_start_position(screen_width, screen_height);
        Ball {
            bounds: Rect::new(starting_point.x, starting_point.y, width, height),
            direction: Ball::random_direction(),
            speed: speed,
        }
    }

    pub fn apply_movement(&mut self, delta_time: f64) {
        let distance = self.speed * delta_time;
        let ball_distance = self.direction.scale_by(distance);
        self.bounds.position = self.bounds.position.add_vector(ball_distance);
    }

    fn random_start_position(screen_width: f64, screen_height: f64) -> Point {
        Point::new(f64::from(screen_width / 2.0), screen_height / 2.0)
    }

    fn random_direction() -> Vector {
        Vector::from(Math::random() * 60.0 - 30.0).normalize()
    }

    pub fn reset(&mut self, screen_width: f64, screen_height: f64) {
        self.direction = Ball::random_direction();
        self.bounds.position = Ball::random_start_position(screen_width, screen_height);
    }
}

#[wasm_bindgen]
pub struct Game {
    pub width: f64,
    pub height: f64,
    center: Point,
    paddle_size: Rect,
    pub speed: f64,
    pub left_paddle: Paddle,
    pub right_paddle: Paddle,
    pub ball: Ball,
    pub started: bool
}

#[wasm_bindgen]
impl Game {
    #[wasm_bindgen(constructor)]
    pub fn new(width: f64, height: f64, speed: f64) -> Game {
        let center = Point::new((width / 2_f64).round(), (height / 2_f64).round());

        let paddle_height = 4.0;
        let paddle_width = 0.5;
        let half_height = paddle_height / 2.0;
        let half_width = paddle_width / 2.0;

        let paddle_size = Rect::new(
            2.0 + half_width,
            center.y - half_height,
            paddle_width,
            paddle_height
        );

        let left_paddle = Paddle::new(paddle_size.position.x, -paddle_height, paddle_width, paddle_height, speed);
        let right_paddle = Paddle::new(
            width - paddle_size.position.x,
            -paddle_height,
            paddle_width,
            paddle_height,
            speed,
        );

        let mut ball = Ball::new(1.0, 1.0, width, height, speed);
        ball.bounds.position.y = -ball.bounds.height;

        Game {
            width: width,
            height: height,
            center: center,
            paddle_size: paddle_size,
            speed: speed,
            left_paddle: left_paddle,
            right_paddle: right_paddle,
            ball: ball,
            started: false
        }
    }

    pub fn start(&mut self) {
        self.next_round();
        self.started = true;
    }

    pub fn stop(&mut self) {
        self.reset_scores();
        self.hide_all();
        self.started = false;
    }

    pub fn next_round(&mut self) {
        self.left_paddle.bounds.position = Point::new(self.paddle_size.position.x, self.paddle_size.position.y);
        self.right_paddle.bounds.position = Point::new(self.width - self.paddle_size.position.x, self.paddle_size.position.y);
        self.ball.reset(self.width, self.height);
    }

    pub fn reset_scores(&mut self) {
        self.left_paddle.score = 0;
        self.right_paddle.score = 0;
    }

    fn hide_all(&mut self) {
        self.left_paddle.bounds.position.y = -self.paddle_size.height;
        self.right_paddle.bounds.position.y = -self.paddle_size.height;
        self.ball.direction = Vector::new(0.0, 0.0);
        self.ball.bounds.position.y = -self.ball.bounds.height;
    }

    pub fn tick(&mut self, delta_time: f64, movement: Option<Movement>) {
        if self.started {
            self.internal_tick(delta_time, movement);
        }
    }

    fn internal_tick(&mut self, delta_time: f64, movement: Option<Movement>) {
        self.ball.apply_movement(delta_time);

        self.check_collisions();

        if self.left_paddle.score >= 11 || self.right_paddle.score >= 11 {
            self.stop();
        }

        if movement.is_some() {
            let vector = self.left_paddle.process_movement(movement.unwrap());
            self.left_paddle.apply_movement(delta_time, vector);
            // self.right_paddle.apply_movement(delta_time, vector);
        }

        // self.right_paddle.apply_movement(delta_time, Vector::new(0.0, 0.0));
    }

    fn check_collisions(&mut self) {
        if self.ball.bounds.max().x < 0.0 {
            self.right_paddle.increment_score();
            self.next_round();
        } else if self.ball.bounds.position.x > self.width {
            self.left_paddle.increment_score();
            self.next_round();
        }

        if self.is_colliding_bounds(&self.ball.bounds) {
            self.ball.direction = Vector::new(self.ball.direction.x, -self.ball.direction.y);
        }

        if self.left_paddle.bounds.overlaps(&self.ball.bounds) {
            let y = Game::hit_factor(&self.left_paddle, &self.ball);
            self.ball.direction = Vector::new(1.0, y).normalize();
        } else if self.right_paddle.bounds.overlaps(&self.ball.bounds) {
            let y = Game::hit_factor(&self.right_paddle, &self.ball);
            self.ball.direction = Vector::new(-1.0, y).normalize();
        }
    }

    fn is_colliding_bounds(&self, rect: &Rect) -> bool {
        rect.position.y < 0.0 || rect.position.y + rect.height > f64::from(self.height)
    }

    fn hit_factor(paddle: &Paddle, ball: &Ball) -> f64 {
        let ball_center = ball.bounds.position.y + (ball.bounds.height / 2.0);
        let paddle_center = paddle.bounds.position.y + (paddle.bounds.height / 2.0);
        (ball_center - paddle_center) / f64::from(paddle.bounds.height)
    }
}
