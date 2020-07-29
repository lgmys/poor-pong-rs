use ggez::{
    event::{EventHandler, KeyCode, KeyMods},
    graphics, nalgebra as na, timer, Context, GameResult,
};

use std::time::{Duration, Instant};

use crate::entities::ball::Ball;
use crate::entities::paddle::{MovementDirection, Paddle};

pub struct MyGame {
    ball: Ball,
    paddle: Paddle,
    last_update: Instant,
    last_draw: Instant,
    fps_last_update: Instant,
    accumulated_time: f32,
    fps_readings: Vec<f32>,
    fixed_time_step: f32,
    movement: MovementDirection,
}

impl MyGame {
    pub fn new(_ctx: &mut Context) -> MyGame {
        let updates_per_second = 100;

        // Load/create resources such as images here.
        MyGame {
            ball: Ball::new(),
            paddle: Paddle::new(),
            last_update: Instant::now(),
            last_draw: Instant::now(),
            accumulated_time: 0.0,
            fps_last_update: Instant::now(),
            fps_readings: vec![],
            fixed_time_step: 1.0 / updates_per_second as f32,
            movement: MovementDirection::None,
        }
    }
}

impl EventHandler for MyGame {
    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: KeyCode,
        _keymods: KeyMods,
        _repeat: bool,
    ) {
        match keycode {
            KeyCode::Up => {
                self.movement = MovementDirection::Up;
            }
            KeyCode::Down => {
                self.movement = MovementDirection::Down;
            }
            _ => {}
        }
    }

    fn key_up_event(&mut self, _ctx: &mut Context, _keycode: KeyCode, _keymods: KeyMods) {
        self.movement = MovementDirection::None;
    }

    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        let diff: Duration = Instant::now() - self.last_update;

        let delta = diff.as_secs_f32();

        self.accumulated_time += delta;

        while self.accumulated_time >= self.fixed_time_step {
            self.paddle.update(&self.movement);

            let mut ball_touches_paddle = false;
            if self.ball.position.x <= 32.0 {
                if (self.ball.position.y >= self.paddle.position.y && self.ball.position.y <= self.paddle.position.y + 128.0) {
                    ball_touches_paddle = true;
                }
            }

            self.ball.update(ball_touches_paddle);

            self.accumulated_time -= self.fixed_time_step;
        }

        self.last_update = Instant::now();

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);

        let fps: Duration = Instant::now() - self.last_draw;
        let fps = 1.0 / fps.as_secs_f32();

        self.fps_readings.push(fps);

        let fps_time_since_update: Duration = Instant::now() - self.fps_last_update;

        if fps_time_since_update.as_secs() >= 5 {
            let mut avg_fps = 0.0;
            for reading in self.fps_readings.iter() {
                avg_fps += reading;
            }

            println!("avg fps (5s): {}", avg_fps / self.fps_readings.len() as f32);

            self.fps_last_update = Instant::now();
            self.fps_readings.clear();
        }

        let ball_sprite = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::stroke(1.0),
            self.ball.position,
            4.0,
            1.0,
            graphics::WHITE,
        )?;

        let paddle_sprite = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::stroke(1.0),
            ggez::graphics::Rect::new(self.paddle.position.x, self.paddle.position.y, 32.0, 128.0),
            graphics::WHITE,
        )?;

        graphics::draw(ctx, &ball_sprite, (na::Point2::new(0.0, 0.0),))?;
        graphics::draw(ctx, &paddle_sprite, (na::Point2::new(0.0, 0.0),))?;

        graphics::present(ctx)?;

        timer::yield_now();

        self.last_draw = Instant::now();

        Ok(())
    }
}