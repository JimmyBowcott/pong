use std::{thread::sleep, time::{Duration, Instant}};

use crate::{player::Player, renderer::Renderer};

pub struct Game {
    player: Player,
}

const FRAME_DURATION_IN_MS: Duration = Duration::from_millis(16);

impl Game {
    pub fn new() -> Self {
        let player = Player::new(0,0);
        Game{player}
    }

    pub fn run(&self, renderer: &mut impl Renderer) {
        loop {
            let frame_start = Instant::now();

            self.render(renderer);

            let elapsed = frame_start.elapsed();
            if elapsed < FRAME_DURATION_IN_MS {
                sleep(FRAME_DURATION_IN_MS - elapsed);
            }
        }
    }

    fn render(&self, renderer: &mut impl Renderer) {
        renderer.clear();
        renderer.put_char(self.player.x, self.player.y, '@');
        renderer.present();
    }
}
