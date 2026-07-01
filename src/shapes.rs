use rand::Rng;
use crate::drawline::render_line;
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


// structs
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}



#[derive(Debug, PartialEq, Eq)]
pub struct Line {
    pub p1: Point,
    pub p2: Point,
}


// #[derive(Debug, PartialEq, Eq, Clone)]
// pub struct Triangle {
//     pub p1: Point,
//     pub p2: Point,
//     pub p3: Point,
// }



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

// #[derive(Debug, PartialEq, Eq)]
// pub struct Circle {
//     pub center: Point,
//     pub radius: i32,
// }


//implementations :

impl Drawable for Point {
    fn draw(&self, canvas: &mut Image) {
        canvas.display(self.x, self.y, self.color());
    }
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

// impl Triangle {
//     pub fn new(p1: &Point, p2: &Point, p3: &Point) -> Self {
//         Self {
//             p1: p1.clone(),
//             p2: p2.clone(),
//             p3: p3.clone(),
//         }
//     }
// }


// impl Circle {
// //implemeeenetee
// }


impl Line {
    pub fn random(w: i32, h: i32) -> Self {
        Self {
            p1: Point::random(w, h),
            p2: Point::random(w, h),
        }
    }
}

impl Drawable for Line {
    fn draw(&self, image: &mut Image) {
        render_line(image, &self.p1, &self.p2, self.color())
    }
}

impl Drawable for Rectangle {
    fn draw(&self, image: &mut Image) {
        let cc = self.color();
        let p3 = Point::new(self.p1.x , self.p2.y);
        let p4 = Point::new(self.p2.x , self.p1.y);

        render_line(image,&self.p1, &p3 , cc.clone());
        render_line(image,&p3, &self.p2 , cc.clone());
        render_line(image,&self.p2, &p4 , cc.clone());
        render_line(image,&p4, &self.p1 , cc.clone());
        //implement Rec draw
    }
}