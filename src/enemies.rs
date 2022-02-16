use crate::{position_to_transform, AppState, Graph};
use bevy::core::FixedTimestep;
use bevy::prelude::*;

const ENEMY_DELAY: f64 = 1.0;

#[derive(Component)]
struct Slime {
	pub position: (usize, usize),
	pub target: (usize, usize),
}

struct SlimeCount(u8);

fn begin_wave(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	mut slime_count: ResMut<SlimeCount>,
	graph: Res<Graph>,
	app_state: Res<State<AppState>>,
) {
	if *app_state.current() == AppState::EnemiesState && slime_count.0 < 5 {
		let start = graph.get_node_coordinates(graph.start).unwrap();
		commands
			.spawn_bundle(SpriteBundle {
				texture: asset_server.load("slime.png"),
				transform: position_to_transform(start.0 as f32, start.1 as f32),
				..Default::default()
			})
			.insert(Slime {
				position: start,
				target: start,
			});
		slime_count.0 += 1
	}
}

fn slime_ai(graph: Res<Graph>, mut slimes: Query<&mut Sprite, With<Slime>>) {
	for mut slime in slimes.iter_mut() {}
}

fn count_slimes(query: Query<&Slime>, app_state: Res<State<AppState>>) {
	if *app_state.current() == AppState::EnemiesState {
		println!("{}", query.iter().len())
	}
}

pub struct EnemiesPlugin;

impl Plugin for EnemiesPlugin {
	fn build(&self, app: &mut App) {
		app.insert_resource(SlimeCount(0)).add_system_set(
			SystemSet::new()
				.with_run_criteria(FixedTimestep::step(ENEMY_DELAY))
				.with_system(begin_wave)
				.with_system(count_slimes),
		);
	}
}
