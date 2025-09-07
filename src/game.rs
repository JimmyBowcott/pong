use std::{
    thread::sleep, time::{Duration, Instant}
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

const FRAME_DURATION_IN_MS: Duration = Duration::from_millis(33);

impl Game {
    pub fn new(screen_width: usize, screen_height: usize) -> Self {
        Game {
            running: true,
            score: 0,
            player: Player::new(screen_width.saturating_sub(2), screen_height / 2),
            enemy_player: Player::new(2, screen_height / 2),
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

            self.handle_input();
            self.ball.update(self.screen_height);

            for paddle in &[&self.player, &self.enemy_player] {
                if paddle.collides(self.ball.x as usize, self.ball.y as usize) {
                    self.ball.bounce_from_paddle(paddle);
                }
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

    fn render(&self, renderer: &mut impl Renderer) {
        renderer.clear();
        self.player.draw(renderer);
        self.enemy_player.draw(renderer);
        self.ball.draw(renderer);
        self.draw_score(renderer);
        renderer.present();
    }

    fn draw_score(&self, renderer: &mut impl Renderer) {
        renderer.draw_text(&format!("Score: {}", self.score), 2, 2);
    }
}
