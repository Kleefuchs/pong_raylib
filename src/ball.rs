use raylib::prelude::*;

pub struct Ball {
    pub pos: Vector2,
    pub delta_speed: Vector2,
    pub radius: f32,
    pub color: Color,
    pub speed_y_prerequisite_speed_x_divider: f32,
    pub speed_x_factor: f32
}

impl Ball {
    pub fn update(&mut self, rl: &mut RaylibHandle, screen_height: i32) {
        self.pos.x += self.delta_speed.x * rl.get_frame_time();
        self.pos.y += self.delta_speed.y * rl.get_frame_time();
        if (self.pos.y <= self.radius) & (self.delta_speed.y < 0.0) {
            self.delta_speed.y *= -1.0;
        }
        if ((self.pos.y + self.radius) >= screen_height as f32) & (self.delta_speed.y > 0.0) {
            self.delta_speed.y *= -1.0;
        }
    }
    pub fn draw(&self, draw:&mut RaylibDrawHandle) {
        draw.draw_circle(self.pos.x as i32, self.pos.y as i32, self.radius, self.color);
    }
}