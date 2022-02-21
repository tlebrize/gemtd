use crate::{
	fit_to_grid, position_to_transform, position_to_translation, vec2_to_position, AppState, Game,
	Graph, SlimeModifier,
};
use bevy::core::FixedTimestep;
use bevy::prelude::*;
use bevy::utils::Duration;
use std::collections::HashMap;

const ENEMY_DELAY: f64 = 1.0;
const POISON_DELAY: f64 = 1.0;

#[derive(Component)]
pub struct Slime {
	pub position: (usize, usize),
	pub target: (usize, usize),
	pub position_index: usize,
	pub velocity: Vec2,
	pub speed: f32, // TODO set base movespeed so we can reduce it with modifiers.
	pub life: usize,
	pub armor: f32,
	pub magic_resistance: f32,
	pub modifiers: SlimeModifier,
}

impl Slime {
	fn get_armor(&self) -> f32 {
		let mut armor = self.armor;
		for (modifier, _) in self.modifiers.armor.iter() {
			armor += *modifier as f32;
		}
		armor
	}

	pub fn take_pure_damage(&mut self, damage: usize) {
		if damage > self.life {
			self.life = 0;
		} else {
			self.life -= damage;
		}
	}

	pub fn take_magic_damage(&mut self, damage: f32) {
		let computed_damage = damage * (1.0 - self.magic_resistance);
		// iter on magic res mods
		self.take_pure_damage(computed_damage as usize);
	}

	pub fn take_physical_damage(&mut self, damage: f32) {
		let computed_damage = (damage
			* (1.0 - (0.052 * self.get_armor()) / (0.9 + 0.048 * self.get_armor().abs())))
			as usize;
		self.take_pure_damage(computed_damage as usize);
	}

	pub fn get_speed_vector(&self) -> Vec2 {
		let mut speed = 100.0;
		for (modifier, _) in self.modifiers.speed.iter() {
			speed *= ((100 - *modifier) as f32) / 100.0;
		}
		if speed < 1.0 {
			speed = 1.0;
		} else if speed > 540.0 {
			speed = 540.0
		}
		Vec2::new(speed, speed)
	}
}

struct SlimeCounter {
	count: usize,
	total_spawned: usize,
}

fn begin_wave(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	mut slime_counter: ResMut<SlimeCounter>,
	graph: Res<Graph>,
	app_state: Res<State<AppState>>,
) {
	if *app_state.current() == AppState::Enemies && slime_counter.total_spawned < 5 {
		let start = graph.get_node_position(graph.start).unwrap();
		commands
			.spawn_bundle(SpriteBundle {
				texture: asset_server.load("slime.png"),
				transform: position_to_transform(start.0 as f32, start.1 as f32)
					.with_scale(Vec3::new(3.0, 3.0, 1.0)),
				..Default::default()
			})
			.insert(Slime {
				position: start,
				target: start,
				position_index: 0,
				velocity: Vec2::new(0.0, 0.0),
				life: 5,
				armor: 0.0,
				magic_resistance: 0.25,
				speed: 100.0,
				modifiers: SlimeModifier::default(),
			});
		slime_counter.total_spawned += 1;
		slime_counter.count += 1;
	}
}

fn slime_pathfinding(graph: Res<Graph>, mut game: ResMut<Game>, mut slimes: Query<&mut Slime>) {
	for mut slime in slimes.iter_mut() {
		if slime.position == slime.target {
			if let Some(target) = graph.next_step(slime.position_index) {
				slime.target = target;
				slime.position_index += 1;
				let target_vector = position_to_translation(target.0 as f32, target.1 as f32)
					- position_to_translation(slime.position.0 as f32, slime.position.1 as f32);
				slime.velocity = Vec2::new(target_vector.x, target_vector.y).normalize()
					* slime.get_speed_vector();
			} else {
				game.lives -= 1;
				slime.life = 0;
			}
		}
	}
}

fn update_slime_position(mut slimes: Query<(&Transform, &mut Slime)>) {
	for (transform, mut slime) in slimes.iter_mut() {
		// TODO because of fit to grid the slimes aren't in the middle of the cells.
		if let Some(position) = fit_to_grid(vec2_to_position(Vec2::new(
			transform.translation.x,
			transform.translation.y,
		))) {
			if slime.target == position {
				slime.position = position;
			}
		}
	}
}

fn slime_movement(time: Res<Time>, mut slimes: Query<(&mut Transform, &Slime)>) {
	for (mut transform, slime) in slimes.iter_mut() {
		transform.translation.x += slime.velocity.x * time.delta_seconds();
		transform.translation.y += slime.velocity.y * time.delta_seconds();
	}
}

fn slime_death(
	mut commands: Commands,
	slimes: Query<(Entity, &Slime)>,
	mut slime_counter: ResMut<SlimeCounter>,
) {
	for (entity, slime) in slimes.iter() {
		if slime.life == 0 {
			commands.entity(entity).despawn_recursive();
			slime_counter.count -= 1;
		}
	}
}

fn end_enemies_state(
	mut slime_counter: ResMut<SlimeCounter>,
	mut app_state: ResMut<State<AppState>>,
	mut game: ResMut<Game>,
) {
	if slime_counter.total_spawned == 5 && slime_counter.count == 0 {
		game.rocks_count = 0;
		app_state.set(AppState::Build).unwrap();
		slime_counter.total_spawned = 0;
	}
}

fn filter_timers(x: HashMap<i32, Duration>, delta: Duration) -> HashMap<i32, Duration> {
	x.into_iter()
		.filter(|&(_, d)| d > delta) // remove mods about to expire
		.map(|(v, d)| (v, d - delta)) // reduce duration for the others
		.collect()
}

fn update_timed_modifiers(time: Res<Time>, mut query: Query<&mut Slime>) {
	for mut slime in query.iter_mut() {
		slime.modifiers.armor = filter_timers(slime.modifiers.armor.clone(), time.delta());
		slime.modifiers.poison = filter_timers(slime.modifiers.poison.clone(), time.delta());
		slime.modifiers.speed = filter_timers(slime.modifiers.speed.clone(), time.delta());
	}
}

fn take_poison_damage(mut query: Query<&mut Slime>) {
	for mut slime in query.iter_mut() {
		let mut damage = 0.0;
		for (value, _) in slime.modifiers.poison.iter_mut() {
			damage += *value as f32;
		}
		if damage > 0.0 {
			slime.take_magic_damage(damage);
		}
	}
}

pub struct EnemiesPlugin;

impl Plugin for EnemiesPlugin {
	fn build(&self, app: &mut App) {
		app.insert_resource(SlimeCounter {
			count: 0,
			total_spawned: 0,
		})
		.add_system_set(
			SystemSet::new()
				.with_run_criteria(FixedTimestep::step(ENEMY_DELAY))
				.with_system(begin_wave),
		)
		.add_system_set(
			SystemSet::new()
				.with_run_criteria(FixedTimestep::step(POISON_DELAY))
				.with_system(take_poison_damage),
		)
		.add_system_set(
			SystemSet::on_update(AppState::Enemies)
				.with_system(slime_pathfinding)
				.with_system(slime_movement)
				.with_system(slime_death)
				.with_system(end_enemies_state)
				.with_system(update_timed_modifiers)
				.with_system(update_slime_position),
		);
	}
}
