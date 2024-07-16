use raylib::prelude::*;

mod paddle;
mod ball;

static SCREEN_WIDTH:i32 = 1280;
static SCREEN_HEIGHT:i32 = 720;

fn handle_collision_paddle_ball(l_paddle:&paddle::Paddle, r_paddle:&paddle::Paddle, ball:&mut ball::Ball) {
    if l_paddle.check_collision_ball(ball) & (ball.delta_speed.x < 0.0) {
        ball.delta_speed.x *= ball.speed_x_factor;
        ball.delta_speed.y = (l_paddle.dest.y + l_paddle.dest.height / 2.0 - ball.pos.y) * -ball.delta_speed.x / ball.speed_y_prerequisite_speed_x_divider;
    }
    if r_paddle.check_collision_ball(ball) & (ball.delta_speed.x > 0.0) {
        ball.delta_speed.x *= ball.speed_x_factor;
        ball.delta_speed.y = (r_paddle.dest.y + r_paddle.dest.height / 2.0 - ball.pos.y) * ball.delta_speed.x / ball.speed_y_prerequisite_speed_x_divider;
    }
}

fn main() {
    let mut l_paddle:paddle::Paddle = paddle::Paddle {
        dest: Rectangle {
            x: 20.0,
            y: (SCREEN_HEIGHT / 2 - 50) as f32,
            width: 20.0,
            height: 100.0
        },
        color: Color::WHITE,
        upkey: KeyboardKey::KEY_W,
        downkey: KeyboardKey::KEY_S,
        speed: 400.0
    };
    let mut r_paddle:paddle::Paddle = paddle::Paddle {
        dest: Rectangle {
            x: (SCREEN_WIDTH - 40) as f32,
            y: (SCREEN_HEIGHT / 2 - 50) as f32,
            width: 20.0,
            height: 100.0
        },
        color: Color::WHITE,
        upkey: KeyboardKey::KEY_UP,
        downkey: KeyboardKey::KEY_DOWN,
        speed: 400.0
    };
    let mut ball:ball::Ball = ball::Ball {
        pos: Vector2 { x: (SCREEN_WIDTH / 2) as f32, y: (SCREEN_HEIGHT / 2) as f32 },
        delta_speed: Vector2 {x: 250.0, y: 250.0},
        radius: 15.0,
        color: Color::WHITE,
        speed_y_prerequisite_speed_x_divider: 50.0,
        speed_x_factor: -1.1
    };
    let (mut rl , thread) = init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Pong")
        .build();
    while rl.window_should_close()==false {
        l_paddle.update(&mut rl, SCREEN_HEIGHT);
        r_paddle.update(&mut rl, SCREEN_HEIGHT);
        ball.update(&mut rl, SCREEN_HEIGHT);
        handle_collision_paddle_ball(&l_paddle, &r_paddle, &mut ball);
        let mut draw = rl.begin_drawing(&thread);
        draw.clear_background(Color::BLACK);
        l_paddle.draw(&mut draw);
        r_paddle.draw(&mut draw);
        ball.draw(&mut draw);
        
    }
}
