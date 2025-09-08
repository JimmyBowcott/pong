use crate::renderer::Renderer;

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

    pub fn collides(&self, x: usize, y: usize) -> bool {
        let start = (self.y.round() as usize).saturating_sub(self.height / 2);
        let end = self.y.round() as usize + self.height / 2;

        x as f32 == self.x && (start..=end).contains(&y)
    }

    pub fn move_speed(&self) -> f32 {
        self.move_speed
    }

    pub fn x(&self) -> usize {
        self.x.round() as usize
    }

    pub fn accelerate(&mut self) {
        self.move_speed *= 1.01;
    }
}
