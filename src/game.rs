use std::{thread::sleep, time::{Duration, Instant}};

use crate::{controller::{InputController, Key}, player::Player, renderer::Renderer};

pub struct Game {
    running: bool,
    player: Player,
    controller: InputController,
    screen_width: usize,
    screen_height: usize,
}

const FRAME_DURATION_IN_MS: Duration = Duration::from_millis(33);

impl Game {
    pub fn new(screen_width: usize, screen_height: usize) -> Self {
        let player = Player::new(screen_width.saturating_sub(2),screen_height / 2);
        let controller = InputController::new();
        Game{running: true, player, controller, screen_width, screen_height}
    }

    pub fn run(&mut self, renderer: &mut impl Renderer) {
        self.running = true;
        loop {
            if !self.running {
                break;
            }

            let frame_start = Instant::now();

            self.handle_input();

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
            return
        }

        match self.controller.direction() {
            Key::Up => self.player.move_up(),
            Key::Down => self.player.move_down(self.screen_height),
            _ => {},
        }
    }

    fn render(&self, renderer: &mut impl Renderer) {
        renderer.clear();
        self.player.draw(renderer);
        renderer.present();
    }
}
