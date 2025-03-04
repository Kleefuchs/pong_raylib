use raylib::prelude::*;

mod paddle;
mod ball;
mod gamestate;

static SCREEN_WIDTH:i32 = 1280;
static SCREEN_HEIGHT:i32 = 720;

fn reset_game(l_paddle:&mut paddle::Paddle, r_paddle:&mut paddle::Paddle, ball:&mut ball::Ball, screen_width:i32, screen_height:i32) {
    l_paddle.generate(true, screen_width, screen_height);
    r_paddle.generate(false, screen_width, screen_height);
    ball.generate(screen_width, screen_height)
}

fn check_and_handle_collision_paddle_ball(l_paddle:&paddle::Paddle, r_paddle:&paddle::Paddle, ball:&mut ball::Ball) {
    if l_paddle.check_collision_ball(ball) & (ball.delta_speed.x < 0.0) {
        ball.delta_speed.x *= ball.speed_x_factor;
        ball.delta_speed.y = (l_paddle.dest.y + l_paddle.dest.height / 2.0 - ball.pos.y) * -ball.delta_speed.x / ball.speed_y_prerequisite_speed_x_divider;
    }
    if r_paddle.check_collision_ball(ball) & (ball.delta_speed.x > 0.0) {
        ball.delta_speed.x *= ball.speed_x_factor;
        ball.delta_speed.y = (r_paddle.dest.y + r_paddle.dest.height / 2.0 - ball.pos.y) * ball.delta_speed.x / ball.speed_y_prerequisite_speed_x_divider;
    }
}

fn handle_win_condition(l_paddle:&mut paddle::Paddle, r_paddle:&mut paddle::Paddle, ball:&mut ball::Ball, screen_width:i32, screen_height:i32) {
    reset_game(l_paddle, r_paddle, ball, screen_width, screen_height);
}

fn handle_l_win_condition(l_paddle:&mut paddle::Paddle, r_paddle:&mut paddle::Paddle, ball:&mut ball::Ball, gamestate:&mut gamestate::Gamestate, screen_width:i32, screen_height:i32) {
    *gamestate = gamestate::Gamestate::LWON;
    handle_win_condition(l_paddle, r_paddle, ball, screen_width, screen_height);
}

fn handle_r_win_condition(l_paddle:&mut paddle::Paddle, r_paddle:&mut paddle::Paddle, ball:&mut ball::Ball, gamestate:&mut gamestate::Gamestate, screen_width:i32, screen_height:i32) {
    *gamestate = gamestate::Gamestate::RWON;
    handle_win_condition(l_paddle, r_paddle, ball, screen_width, screen_height);
}


fn check_and_handle_goal(ball:&mut ball::Ball, l_paddle:&mut paddle::Paddle, r_paddle:&mut paddle::Paddle, gamestate:&mut gamestate::Gamestate, screen_width:i32, screen_height:i32) {
    if (ball.pos.x + ball.radius) < 0.0 {
        ball.generate(screen_width, screen_height);
        r_paddle.points += 1;
        if r_paddle.points >= 9 {
            handle_r_win_condition(l_paddle, r_paddle, ball, gamestate, screen_width, screen_height)
        }
    }
    if (ball.pos.x - ball.radius) > screen_width as f32 {
        ball.generate(screen_width, screen_height);
        l_paddle.points += 1;
        if l_paddle.points >= 9 {
            handle_l_win_condition(l_paddle, r_paddle, ball, gamestate, screen_width, screen_height)
        }
    }
}

fn draw_points(draw:&mut RaylibDrawHandle, font: & WeakFont, l_paddle:&paddle::Paddle, r_paddle:&paddle::Paddle, screen_width:i32) {
    draw.draw_text_pro(font, l_paddle.points.to_string().as_str(), Vector2 {x: 400.0 - draw.measure_text(l_paddle.points.to_string().as_str(), 60) as f32 / 2.0, y: 100.0}, Vector2 {x: 0.0, y: 0.0}, 0.0, 60.0, 1.0, Color::WHITE);
    draw.draw_text_pro(font, r_paddle.points.to_string().as_str(), Vector2 {x: screen_width as f32 -400.0 - draw.measure_text(r_paddle.points.to_string().as_str(), 60) as f32 / 2.0, y: 100.0}, Vector2 {x: 0.0, y: 0.0}, 0.0, 60.0, 1.0, Color::WHITE);
}

fn main() {
    let mut against_ai = false;
    let mut gamestate:gamestate::Gamestate = gamestate::Gamestate::MENU;
    let mut l_paddle:paddle::Paddle = paddle::Paddle::new();
    l_paddle.generate(true, SCREEN_WIDTH, SCREEN_HEIGHT);
    let mut r_paddle:paddle::Paddle = paddle::Paddle::new();
    r_paddle.generate(false, SCREEN_WIDTH, SCREEN_HEIGHT);
    let mut ball:ball::Ball = ball::Ball::new();
    ball.generate(SCREEN_WIDTH, SCREEN_HEIGHT);
    let (mut rl , thread) = init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Pong")
        .build();
    let standard_font: WeakFont = rl.get_font_default();
    while rl.window_should_close()==false {
        match gamestate {
            gamestate::Gamestate::MENU => {
                if rl.is_key_pressed(KeyboardKey::KEY_ONE) {
                    gamestate = gamestate::Gamestate::GAME;
                    against_ai = false;
                    reset_game(&mut l_paddle, &mut r_paddle, &mut ball, SCREEN_WIDTH, SCREEN_HEIGHT);
                }
                if rl.is_key_pressed(KeyboardKey::KEY_TWO) {
                    gamestate = gamestate::Gamestate::GAME;
                    against_ai = true;
                    reset_game(&mut l_paddle, &mut r_paddle, &mut ball, SCREEN_WIDTH, SCREEN_HEIGHT);
                }
            }
            gamestate::Gamestate::GAME => {
                if rl.is_key_pressed(KeyboardKey::KEY_SPACE) {
                    gamestate = gamestate::Gamestate::PAUSE;
                }
                l_paddle.update(&mut rl, &ball, against_ai, SCREEN_HEIGHT);
                r_paddle.update(&mut rl, &ball, against_ai, SCREEN_HEIGHT);
                ball.update(&mut rl, SCREEN_HEIGHT);
                check_and_handle_collision_paddle_ball(&l_paddle, &r_paddle, &mut ball);
                check_and_handle_goal(&mut ball, &mut l_paddle, &mut r_paddle, &mut gamestate, SCREEN_WIDTH, SCREEN_HEIGHT);
            }
            gamestate::Gamestate::PAUSE => {
                if rl.is_key_pressed(KeyboardKey::KEY_SPACE) {
                    gamestate = gamestate::Gamestate::GAME;
                }
            }
            gamestate::Gamestate::LWON => {
                if rl.is_key_pressed(KeyboardKey::KEY_SPACE) {
                    gamestate = gamestate::Gamestate::GAME;
                }
            }
            gamestate::Gamestate::RWON => {
                if rl.is_key_pressed(KeyboardKey::KEY_SPACE) {
                    gamestate = gamestate::Gamestate::GAME;
                }
            }
        }
        if rl.is_key_pressed(KeyboardKey::KEY_M) {
            gamestate = gamestate::Gamestate::MENU;
        }
        let mut draw = rl.begin_drawing(&thread);
        match gamestate {
            gamestate::Gamestate::MENU => {
                draw.clear_background(Color::BLACK);
            }
            gamestate::Gamestate::GAME => {
                draw.clear_background(Color::BLACK);
                l_paddle.draw(&mut draw);
                r_paddle.draw(&mut draw);
                ball.draw(&mut draw);
                draw_points(&mut draw, &standard_font, &l_paddle, &r_paddle, SCREEN_WIDTH);
            }
            gamestate::Gamestate::PAUSE => {
                draw.clear_background(Color::BLACK);
                l_paddle.draw(&mut draw);
                r_paddle.draw(&mut draw);
                ball.draw(&mut draw);
                draw_points(&mut draw, &standard_font, &l_paddle, &r_paddle, SCREEN_WIDTH);
            }
            gamestate::Gamestate::LWON => {
                draw.clear_background(Color::BLACK);
                draw.draw_text_pro(&standard_font, "Left Player Won", Vector2 {x: (SCREEN_WIDTH / 2 - draw.measure_text("Left Player Won", 40) / 2) as f32, y: 100.0}, Vector2 {x: 0.0, y: 0.0}, 0.0, 40.0, 1.0, Color::WHITE);
            }
            gamestate::Gamestate::RWON => {
                draw.clear_background(Color::BLACK);
                draw.draw_text_pro(&standard_font, "Right Player Won", Vector2 {x: (SCREEN_WIDTH / 2 - draw.measure_text("Right Player Won", 40) / 2) as f32, y: 100.0}, Vector2 {x: 0.0, y: 0.0}, 0.0, 40.0, 1.0, Color::WHITE);
            }
        }
    }
}
