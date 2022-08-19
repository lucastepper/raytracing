use super::*;

pub struct Camera {
    width: usize,
    height: usize,
    origin: Point,
    screen_bottom_left: Point,
    dx: f32,
    dy: f32,
}
impl Camera {
    pub fn new(
        origin: Point,
        width: u32,
        height: u32,
        focal_len: f32,
        screen_width: f32,
    ) -> Camera {
        let screen_height = screen_width / width as f32 * height as f32;
        // generate the rays
        let dx = screen_width / width as f32;
        let dy = screen_height / height as f32;
        let screen_bottom_left = Point::new(-screen_width / 2., -screen_height / 2., -focal_len);
        Camera {
            width: width as usize,
            height: height as usize,
            origin: origin,
            screen_bottom_left,
            dx,
            dy,
        }
    }

    pub fn gen_ray(&self, x: f32, y: f32) -> Ray {
        let pixel_pos = Point::new(
            self.screen_bottom_left.x + x * self.dx,
            self.screen_bottom_left.y + y * self.dy,
            self.screen_bottom_left.z,
        );
        Ray::new(self.origin.clone(), pixel_pos - self.origin)
    }
}
