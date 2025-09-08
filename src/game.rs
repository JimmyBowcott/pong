use std::{
    thread::sleep,
    time::{Duration, Instant},
};

use crate::{
    ball::Ball,
    controller::{InputController, Key},
    player::Player,
    renderer::Renderer,
};

pub struct Game {
    running: bool,
    score: usize,
    player: Player,
    enemy_player: Player,
    ball: Ball,
    controller: InputController,
    screen_width: usize,
    screen_height: usize,
}

const FRAME_DURATION_IN_MS: Duration = Duration::from_millis(16);

impl Game {
    pub fn new(screen_width: usize, screen_height: usize) -> Self {
        Game {
            running: true,
            score: 0,
            player: Player::new(screen_width.saturating_sub(2), screen_height / 2, 0.325),
            enemy_player: Player::new(2, screen_height / 2, 0.25),
            ball: Ball::new(screen_width / 2, screen_height / 2),
            controller: InputController::new(),
            screen_width,
            screen_height,
        }
    }

    pub fn run(&mut self, renderer: &mut impl Renderer) {
        self.running = true;
        loop {
            if !self.running {
                break;
            }

            let frame_start = Instant::now();
            let mut ball_hit = false;

            self.handle_input();
            self.move_enemy();

            for paddle in &[&self.player, &self.enemy_player] {
                if paddle.collides(&self.ball) {
                    self.ball.bounce_from_paddle(paddle);
                    ball_hit = true;
                }
            }

            self.ball.update(self.screen_height);

            if ball_hit {
                self.enemy_player.accelerate();
                self.ball.accelerate();
            }

            if self.ball.at_left_edge() {
                self.score += 1;
                self.ball.reset(self.screen_width, self.screen_height);
            }

            if self.ball.at_right_edge(self.screen_width) {
                self.running = false;
            }

            self.render(renderer);
            let elapsed = frame_start.elapsed();
            if elapsed < FRAME_DURATION_IN_MS {
                sleep(FRAME_DURATION_IN_MS - elapsed);
            }
        }
    }

    pub fn handle_input(&mut self) {
        self.controller.poll();

        if self.controller.should_exit() {
            self.running = false;
            return;
        }

        match self.controller.direction() {
            Key::Up => self.player.move_up(),
            Key::Down => self.player.move_down(self.screen_height),
            _ => {}
        }
    }

    fn move_enemy(&mut self) {
        if self.ball.is_moving_to(self.enemy_player.x()) && self.ball.is_near(self.enemy_player.x()) {
            let target_y = self.predict_ball_y();
            self.enemy_move_toward(target_y);
        }
    }

    fn predict_ball_y(&self) -> f32 {
        let distance_x = (self.enemy_player.x as f32 - self.ball.x) / self.ball.v_x();
        let mut target_y = self.ball.y + self.ball.v_y() * distance_x;

        while target_y < 0.0 || target_y > self.screen_height as f32 {
            if target_y < 0.0 {
                target_y = -target_y;
            }
            if target_y > self.screen_height as f32 {
                target_y = 2.0 * self.screen_height as f32 - target_y;
            }
        }

        target_y
    }

    fn enemy_move_toward(&mut self, target_y: f32) {
        if self.enemy_player.y as f32 + self.enemy_player.move_speed() < target_y {
            self.enemy_player.move_down(self.screen_height);
        } else if self.enemy_player.y as f32 - self.enemy_player.move_speed() > target_y {
            self.enemy_player.move_up();
        }
    }

    fn render(&self, renderer: &mut impl Renderer) {
        renderer.clear();
        self.ball.draw(renderer);
        self.player.draw(renderer);
        self.enemy_player.draw(renderer);
        self.draw_score(renderer);
        renderer.present();
    }

    fn draw_score(&self, renderer: &mut impl Renderer) {
        renderer.draw_text(&format!("Score: {}", self.score), 2, 2);
    }
}
