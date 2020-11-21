pub use super::dvec2::*;

#[derive(Debug)]
pub struct Particle {
	pub pos: dvec2,
	pub vel: dvec2,
	pub mass: f64,
}
