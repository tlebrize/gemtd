use bevy::utils::Duration;
use std::collections::HashMap;

#[derive(std::cmp::PartialEq, Clone)]
pub struct Cleave {
	pub range: f32,
	pub damage: f32,
}

#[derive(std::cmp::PartialEq, Clone, Default)]
pub struct TowerModifier {
	pub attack_speed: Vec<f32>,
	pub apply_armor: HashMap<i32, Duration>,
	pub apply_poison: HashMap<i32, Duration>,
	pub apply_speed: HashMap<i32, Duration>,
	pub cleave: Option<Cleave>,
	pub target_count: Option<usize>,
}

impl TowerModifier {
	pub fn get_target_count(&self) -> usize {
		self.target_count.or(Some(1)).unwrap()
	}
}

#[derive(std::cmp::PartialEq, Clone, Default)]
pub struct TowerAuras {
	pub attack_speed: Vec<(f32, f32)>,
}

#[derive(std::cmp::PartialEq, Clone, Default)]
pub struct SlimeModifier {
	pub armor: HashMap<i32, Duration>,
	pub poison: HashMap<i32, Duration>,
	pub speed: HashMap<i32, Duration>,
}
