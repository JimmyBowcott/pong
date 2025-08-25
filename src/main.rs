use crate::renderer::TerminalRenderer;

mod game;
mod player;
mod renderer;

fn main() {
    let game = game::Game::new();
    let mut renderer = TerminalRenderer::new();
    game.run(&mut renderer);
}
