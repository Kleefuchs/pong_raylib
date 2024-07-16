use raylib::prelude::*;

use crate::ball;

pub struct Paddle {
    pub dest: Rectangle,
    pub color: Color,
    pub upkey: KeyboardKey,
    pub downkey: KeyboardKey,
    pub speed: f32
}

#[allow(dead_code)]
impl Paddle {
    pub fn check_collision_ball(&self, ball:&mut ball::Ball) -> bool {
        if self.dest.check_collision_circle_rec(ball.pos, ball.radius) {
            return true;
        }
        return false;
    }
    pub fn update(&mut self, rl:&mut RaylibHandle, screen_height:i32) {
        if (rl.is_key_down(self.upkey)) & (self.dest.y > 0.0) {
            self.dest.y -= self.speed * rl.get_frame_time();
        }
        if (rl.is_key_down(self.downkey)) & ((self.dest.y + self.dest.height) < screen_height as f32) {
            self.dest.y += self.speed * rl.get_frame_time();
        }
    }
    pub fn draw(&self, draw:&mut RaylibDrawHandle) {
        draw.draw_rectangle_rec(self.dest, self.color);
    }
}