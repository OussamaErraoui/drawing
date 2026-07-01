use rand::Rng;
use raster::{Color, Image};

// 1. Define the traits expected by main.rs
pub trait Drawable {
    fn draw(&self, canvas: &mut Image);
    
    // We can rename internal variables here
    fn color(&self) -> Color {
        let mut generator = rand::thread_rng();
        Color {
            r: generator.gen_range(0..=255),
            g: generator.gen_range(0..=255),
            b: generator.gen_range(0..=255),
            a: generator.gen_range(0..=255),
        }
    }
}

pub trait Displayable {
    fn display(&mut self, x: i32, y: i32, color: Color);
}

// 2. Implement the Point
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn random(max_width: i32, max_height: i32) -> Self {
        let mut rng = rand::thread_rng();
        Self {
            x: rng.gen_range(0..max_width),
            y: rng.gen_range(0..max_height),
        }
    }
}

impl Drawable for Point {
    fn draw(&self, canvas: &mut Image) {
        canvas.display(self.x, self.y, self.color());
    }
}

//linee
#[derive(Debug, PartialEq, Eq)]
pub struct Line {
    pub p1: Point,
    pub p2: Point,
}


#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Triangle {
    pub p1: Point,
    pub p2: Point,
    pub point3: Point,
}
impl Triangle {
    pub fn new(p1: &Point, p2: &Point, point3: &Point) -> Self {
        Self {
            p1: p1.clone(),
            p2: p2.clone(),
            point3: point3.clone(),
        }
    }
}


#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Rectangle {
    pub p1: Point,
    pub p2: Point,
}
impl Rectangle {
    pub fn new(p1: &Point, p2: &Point) -> Self {
        Self {
            p1: p1.clone(),
            p2: p2.clone(),
        }
    }
}