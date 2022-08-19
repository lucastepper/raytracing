mod utils;
use utils::{enlarge_image, Color, Point, Ray};
mod objects;
mod render_image;
use image::RgbImage;

fn main() {
    // Parameters
    let height = 200;
    let aspect_ratio = 16. / 9.;
    let width = (height as f32 * aspect_ratio) as u32;
    let camera_origin = Point::new(0., 0., 0.);
    let camera_focal_len = 1.0;
    let camera_screen_width = 4.0;
    // init image
    let mut image = RgbImage::new(width, height);
    // define camera
    let camera = render_image::Camera::new(
        camera_origin,
        width,
        height,
        camera_focal_len,
        camera_screen_width,
    );
    // shoot rays
    for x in 0..width {
        for y in 0..height {
            let _ray = camera.gen_ray(x as f32, y as f32);
            let color = Color::new(0.9, 0.2, 0.5);
            let mut pixel = image.get_pixel_mut(x, y);
            color.write_pixel(&mut pixel);
        }
    }
    let image = enlarge_image(&image, 8);
    image.save("image.png").unwrap();
}
