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