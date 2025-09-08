use crate::{ball::Ball, renderer::Renderer};

pub struct Player {
    pub x: f32,
    pub y: f32,
    pub height: usize,
    move_speed: f32,
}

impl Player {
    pub fn new(x: usize, y: usize, move_speed: f32) -> Self {
        Self {
            x: x as f32,
            y: y as f32,
            height: 10,
            move_speed,
        }
    }

    pub fn draw(&self, renderer: &mut impl Renderer) {
        let start = (self.y.round() as usize).saturating_sub(self.height / 2);
        let end = self.y.round() as usize + self.height / 2;

        for row in start..=end {
            renderer.put_char(self.x.round() as usize, row, '█');
        }
    }

    pub fn move_up(&mut self) {
        if self.y > (self.height / 2) as f32 {
            self.y -= self.move_speed;
        }
    }

    pub fn move_down(&mut self, screen_height: usize) {
        if self.y + ((self.height / 2) as f32) < (screen_height - 1) as f32 {
            self.y += self.move_speed;
        }
    }

    pub fn collides(&self, ball: &Ball) -> bool {
        let (next_x, next_y) = ball.next_position();

        let min_x = ball.x.min(next_x);
        let max_x = ball.x.max(next_x);
        let min_y = ball.y.min(next_y);
        let max_y = ball.y.max(next_y);

        let paddle_top = self.y - self.height as f32 / 2.0;
        let paddle_bottom = self.y + self.height as f32 / 2.0;
        let paddle_left = self.x as f32;
        let paddle_right = self.x as f32;

        max_x >= paddle_left &&
        min_x <= paddle_right &&
        max_y >= paddle_top &&
        min_y <= paddle_bottom
    }

    pub fn move_speed(&self) -> f32 {
        self.move_speed
    }

    pub fn x(&self) -> usize {
        self.x.round() as usize
    }

    pub fn accelerate(&mut self) {
        self.move_speed += 0.1;
    }
}
