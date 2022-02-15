use bevy::prelude::*;

#[derive(Component)]
pub struct Tile;

#[derive(Component)]
pub struct Wall;

#[derive(std::cmp::PartialEq, Clone)]
pub enum CellContent {
	Empty,
	Limit,
	Start,
	End,
	Rock,
}

#[derive(Component)]
pub struct Cell {
	pub content: CellContent,
	pub position: [usize; 2],
	pub node_id: u32,
}

pub struct Game {
	pub grid: Vec<Vec<Entity>>,
}
