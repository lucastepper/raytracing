use super::*;

struct Background {
    width: f32,
    height: f32,
    color: Color,
}
impl Background {
    fn new(width: u32, heigt: u32, color: Option<Color>) -> Background {
        // put default values for the colors here
        let color = match color {
            Some(c) => c,
            None => Color::new(0., 0., 0.3),
        };
        Background {
            width: width as f32,
            height: heigt as f32,
            color,
        }
    }

    fn get_color(&self) -> Color {
        self.color.clone()
    }
}
