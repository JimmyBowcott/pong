use crate::{player::Player, renderer::Renderer};
use rand::random;

pub struct Ball {
    pub x: f32,
    pub y: f32,
    v_x: f32,
    v_y: f32,
}

impl Ball {
    pub fn new(x: usize, y: usize) -> Self {
        Self {
            x: x as f32,
            y: y as f32,
            v_x: 0.5,
            v_y: random::<f32>() * 0.15,
        }
    }

    pub fn draw(&self, renderer: &mut impl Renderer) {
        renderer.put_char(self.x.round() as usize, self.y.round() as usize, '⬤');
    }

    pub fn update(&mut self, screen_height: usize) {
        self.apply_velocity();

        if self.is_at_edges(screen_height) {
            self.bounce();
        }
    }

    pub fn bounce_from_paddle(&mut self, paddle: &Player) {
        self.v_x = -self.v_x;

        let relative_y = (self.y as f32 - paddle.y as f32) / (paddle.height as f32 / 2.0);
        self.v_y = relative_y * self.v_x.abs() as f32;
    }

    pub fn reset(&mut self, screen_width: usize, screen_height: usize) {
        self.x = (screen_width / 2) as f32;
        self.y = (screen_height / 2) as f32;
        self.v_x = self.v_x.abs();
        self.v_y = random::<f32>() * 0.3;
    }

    pub fn at_left_edge(&self) -> bool {
        self.x == 0.0
    }

    pub fn at_right_edge(&self, screen_width: usize) -> bool {
        self.x.round() > screen_width as f32
    }

    pub fn is_moving_to(&self, x: usize) -> bool {
       self.x > (x as f32) && self.v_x < 0.0 || self.x < (x as f32) && self.v_x > 0.0
    }
 
    pub fn v_x(&self) -> f32 {
        self.v_x
    }

    pub fn v_y(&self) -> f32 {
        self.v_y
    }

    pub fn is_near(&self, x: usize) -> bool {
        (self.x - x as f32).abs() < 55.0
    }

    pub fn accelerate(&mut self) {
        self.v_x += 0.1;
    }

    pub fn next_position(&self) -> (f32, f32) {
        (self.x + self.v_x, self.y + self.v_y)
    }

    fn apply_velocity(&mut self) {
        self.x += self.v_x;
        self.y += self.v_y;
    }

    fn is_at_edges(&self, screen_height: usize) -> bool {
        self.y <= 0.0 || self.y >= screen_height as f32
    }

    fn bounce(&mut self) {
        self.v_y = -self.v_y
    }
}
