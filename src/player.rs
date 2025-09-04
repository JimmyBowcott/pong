use crate::renderer::Renderer;

pub struct Player {
    pub x: usize,
    pub y: usize,
    pub height: usize,
}

impl Player {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y, height: 10 }
    }

    pub fn draw(&self, renderer: &mut impl Renderer) {
        let start = self.y.saturating_sub(self.height / 2);
        let end = self.y + self.height / 2;

        for row in start..=end {
            renderer.put_char(self.x, row, '|');
        }
    }

    pub fn move_up(&mut self) {
        if self.y > self.height / 2 {
            self.y -= 1;
        }
    }

    pub fn move_down(&mut self, screen_height: usize) {
        if self.y + self.height / 2 < screen_height - 1 {
            self.y += 1;
        }
    }
 
    pub fn collides(&self, x: usize, y: usize) -> bool {
        let start = self.y.saturating_sub(self.height / 2);
        let end = self.y + self.height / 2;

        x == self.x && (start..=end).contains(&y)
    }
}
