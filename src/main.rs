#[crate_id="rust-story#0.0.1"];

extern mod sdl2;
pub mod game;

pub fn main() {
	let mut story = ::game::Game::new();
	story.start();
}
