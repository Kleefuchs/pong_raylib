use raylib::prelude::*;
use crate::ball;

pub struct Paddle {
    pub dest: Rectangle,
    pub color: Color,
    pub upkey: KeyboardKey,
    pub downkey: KeyboardKey,
    pub speed: f32,
    pub points: u8
}

#[allow(dead_code)]
impl Paddle {
    pub fn new() -> Paddle {
        let paddle:Paddle = Paddle {
            dest: Rectangle {
                x: 0.0,
                y: 0.0,
                width: 0.0,
                height: 0.0
            },
            color: Color::RED,
            upkey: KeyboardKey::KEY_NULL,
            downkey: KeyboardKey::KEY_NULL,
            speed: 0.0,
            points: 0
        };
        return  paddle;
    }
    pub fn generate(&mut self, is_left:bool, screen_width:i32, screen_height:i32) {
        let paddle:Paddle = Paddle {
            dest: Rectangle {
                x: if is_left {
                    20.0
                } else {
                    (screen_width - 40) as f32
                },
                y: (screen_height / 2 - 50) as f32,
                width: 20.0,
                height: 100.0
            },
            color: Color::WHITE,
            upkey: if is_left {
                KeyboardKey::KEY_W
            } else {
                KeyboardKey::KEY_UP
            },
            downkey: if is_left {
                KeyboardKey::KEY_S
            } else {
                KeyboardKey::KEY_DOWN
            },
            speed: 400.0,
            points: 0
        };
        *self = paddle;
    }
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