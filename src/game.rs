use crate::{position_to_transform, AppState, Graph, GRID_SIZE, TILE_SIZE};
use bevy::prelude::*;

#[derive(Component)]
pub struct Tile;

#[derive(std::cmp::PartialEq, Clone, Debug)]
pub enum CellContent {
	Empty,
	Limit,
	Start,
	End,
	Rock,
	Tower(Entity),
}

#[derive(Component)]
pub struct Cell {
	pub content: CellContent,
	pub position: (usize, usize),
	pub node_id: u32,
}

#[derive(Default)]
pub struct Game {
	pub grid: Vec<Vec<Entity>>,
	pub rocks_count: u8,
	pub lives: u8,
}

fn init_game(mut commands: Commands, mut game: ResMut<Game>, mut graph: ResMut<Graph>) {
	game.lives = 10;

	let size = GRID_SIZE as usize;
	for y in 0..size {
		game.grid.push(vec![]);
		for x in 0..size {
			let (content, color, walkable) = if x == 0 || y == 0 || x == size - 1 || y == size - 1 {
				(CellContent::Limit, Color::WHITE, false)
			} else if x == 1 && y == 1 {
				(CellContent::Start, Color::GREEN, true)
			} else if x == size - 2 && y == size - 2 {
				(CellContent::End, Color::RED, true)
			} else {
				(CellContent::Empty, Color::BLACK, true)
			};

			let node_id = graph.add(walkable, x, y);

			let cell = Cell {
				content: content.clone(),
				position: (x, y),
				node_id,
			};

			let entity = commands
				.spawn_bundle(SpriteBundle {
					sprite: Sprite {
						color,
						custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
						..Default::default()
					},
					transform: position_to_transform(x as f32, y as f32),
					..Default::default()
				})
				.insert(cell)
				.id();
			game.grid[y].push(entity);

			match content {
				CellContent::Start => graph.start = node_id,
				CellContent::End => graph.end = node_id,
				_ => {}
			}
		}
	}
}

fn update_cell_sprites(mut query: Query<(&Cell, &mut Sprite)>) {
	for (cell, mut sprite) in query.iter_mut() {
		match cell.content {
			CellContent::Limit => sprite.color = Color::WHITE,
			CellContent::Start => sprite.color = Color::GREEN,
			CellContent::End => sprite.color = Color::RED,
			CellContent::Rock => sprite.color = Color::GRAY,
			CellContent::Tower(_) => sprite.color = Color::GRAY,
			_ => {}
		}
	}
}

fn handle_new_rock_placed(
	mut rock_placed: EventReader<RockPlacedEvent>,
	mut game: ResMut<Game>,
	mut app_state: ResMut<State<AppState>>,
) {
	for _ in rock_placed.iter() {
		game.rocks_count += 1;
		if game.rocks_count >= 5 {
			app_state.set(AppState::Select).unwrap();
		}
	}
}

pub struct RockPlacedEvent {
	pub entity: Entity,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
	fn build(&self, app: &mut App) {
		app.insert_resource(Game::default())
			.add_event::<RockPlacedEvent>()
			.add_startup_system(init_game)
			.add_system(update_cell_sprites)
			.add_system_set(
				SystemSet::on_update(AppState::Build).with_system(handle_new_rock_placed),
			);
	}
}
