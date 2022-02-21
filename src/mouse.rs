use crate::{
	fit_to_grid, vec2_to_position, AppState, Cell, CellContent, Game, Graph, NewPathEvent,
	RockPlacedEvent, TemporaryTower, Tower, UpdateUIEvent, WINDOW_HEIGHT, WINDOW_WIDTH,
};
use bevy::input::{mouse::MouseButtonInput, ElementState};
use bevy::prelude::*;

struct MouseState {
	position: Vec2,
	pressed: bool,
	pressed_read: bool,
	width: f32,
	height: f32,
}

impl MouseState {
	fn new(width: f32, height: f32) -> Self {
		MouseState {
			position: Vec2::new(0.0, 0.0),
			pressed: false,
			pressed_read: true,
			width,
			height,
		}
	}

	fn update(&mut self, position: Vec2) {
		self.position.x = position.x - (self.width / 2.0);
		self.position.y = position.y - (self.height / 2.0);
	}
}

fn handle_mouse_events(
	mut cursor_moved_events: EventReader<CursorMoved>,
	mut mouse_button_input_events: EventReader<MouseButtonInput>,
	mut state: ResMut<MouseState>,
) {
	let cursor_event = cursor_moved_events.iter().last();
	if let Some(moved) = cursor_event {
		state.update(moved.position)
	}

	let buttons_event = mouse_button_input_events.iter().last();
	if let Some(mouse_buttons) = buttons_event {
		let had_changes = match (mouse_buttons.state, state.pressed) {
			(ElementState::Pressed, false) => Some(true),
			(ElementState::Released, true) => Some(false),
			(_, _) => None,
		};
		if let Some(change) = had_changes {
			state.pressed_read = false;
			state.pressed = change;
		}
	}
}

fn handle_tooltip_hoover(
	mouse: Res<MouseState>,
	mut update_ui: EventWriter<UpdateUIEvent>,
	game: Res<Game>,
	cells: Query<&Cell>,
) {
	if let Some((x, y)) = fit_to_grid(vec2_to_position(mouse.position)) {
		let entity = game.grid[y][x];
		if let Ok(cell) = cells.get(entity) {
			if let CellContent::Tower(_) = cell.content {
				update_ui.send(UpdateUIEvent {
					position: cell.position,
				});
			}
		}
	}
}

fn handle_build_click(
	mut mouse: ResMut<MouseState>,
	game: Res<Game>,
	mut graph: ResMut<Graph>,
	mut cells: Query<(Entity, &mut Cell)>,
	mut new_path: EventWriter<NewPathEvent>,
	mut rock_placed: EventWriter<RockPlacedEvent>,
) {
	if mouse.pressed && !mouse.pressed_read {
		mouse.pressed_read = true;
		if let Some((x, y)) = fit_to_grid(vec2_to_position(mouse.position)) {
			let entity = game.grid[y][x];
			if let Ok((entity, mut cell)) = cells.get_mut(entity) {
				if cell.content == CellContent::Empty {
					cell.content = CellContent::Rock;
					graph.set_node_walkability(cell.node_id, false);
				} else {
					return;
				}

				if graph.bfs() {
					new_path.send(NewPathEvent(graph.path.as_ref().unwrap().to_vec()));
					rock_placed.send(RockPlacedEvent { entity });
				} else {
					cell.content = CellContent::Empty;
					graph.set_node_walkability(cell.node_id, true);
				}
			}
		}
	}
}

fn handle_select_click(
	mut commands: Commands,
	mut mouse: ResMut<MouseState>,
	game: Res<Game>,
	mut temporary_towers: Query<(Entity, &Tower), With<TemporaryTower>>,
	mut cells: Query<&mut Cell>,
	mut app_state: ResMut<State<AppState>>,
) {
	if mouse.pressed && !mouse.pressed_read {
		mouse.pressed_read = true;
		let mut found = false;
		if let Some((x, y)) = fit_to_grid(vec2_to_position(mouse.position)) {
			for (_, tower) in temporary_towers.iter_mut() {
				if tower.position == (x, y) {
					found = true;
				}
			}

			if found {
				for (entity, tower) in temporary_towers.iter_mut() {
					if tower.position == (x, y) {
						commands.entity(entity).remove::<TemporaryTower>();
					} else {
						let (tx, ty) = tower.position;
						cells.get_mut(game.grid[ty][tx]).unwrap().content = CellContent::Rock;
						commands.entity(entity).despawn_recursive();
					}
				}
				app_state.set(AppState::Enemies).unwrap();
			}
		}
	}
}

fn init_mouse(mut windows: ResMut<Windows>) {
	windows
		.get_primary_mut()
		.unwrap()
		.set_cursor_lock_mode(true);
}

pub struct MousePlugin;

impl Plugin for MousePlugin {
	fn build(&self, app: &mut App) {
		app.insert_resource(MouseState::new(WINDOW_WIDTH, WINDOW_HEIGHT))
			.add_startup_system(init_mouse)
			.add_system(handle_mouse_events)
			.add_system(handle_tooltip_hoover)
			.add_system_set(SystemSet::on_update(AppState::Build).with_system(handle_build_click))
			.add_system_set(
				SystemSet::on_update(AppState::Select).with_system(handle_select_click),
			);
	}
}
