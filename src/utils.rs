use image::{Rgb, RgbImage};
use std::ops;

#[derive(Copy, Clone)]
pub struct Color {
    red: f32,
    green: f32,
    blue: f32,
}
impl Color {
    pub fn new(red: f32, green: f32, blue: f32) -> Color {
        Color { red, green, blue }
    }

    pub fn write_pixel(&self, pixel: &mut Rgb<u8>) {
        pixel[0] = (self.red.min(1.).max(0.) * 255.) as u8;
        pixel[1] = (self.green.min(1.).max(0.) * 255.) as u8;
        pixel[2] = (self.blue.min(1.).max(0.) * 255.) as u8;
    }
}

// dont use ndarray for points to allocate the data on the stack
#[derive(Debug, Copy, Clone)]
pub struct Point {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
impl ops::Add<Point> for Point {
    type Output = Point;

    fn add(mut self, rhs: Point) -> Point {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
        self
    }
}
impl ops::Sub<Point> for Point {
    type Output = Point;

    fn sub(mut self, rhs: Point) -> Point {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
        self
    }
}
impl Point {
    pub fn new(x: f32, y: f32, z: f32) -> Point {
        Point { x, y, z }
    }
}

pub struct Ray {
    origin: Point,
    direction: Point,
}
impl Ray {
    pub fn new(origin: Point, direction: Point) -> Ray {
        Ray {
            origin,
            direction: direction,
        }
    }
}

pub fn enlarge_image(image: &RgbImage, scale: u32) -> RgbImage {
    let mut new_image = RgbImage::new(scale * image.width(), scale * image.height());
    for (x, y, pixel) in image.enumerate_pixels() {
        for i in 0..scale {
            for j in 0..scale {
                new_image.put_pixel(scale * x + i, scale * y + j, pixel.clone());
            }
        }
    }
    new_image
}
