use crate::{flat_distance, AppState, Projectile, Slime, TemporaryTower, Tower};
use bevy::prelude::*;
use bevy::utils::Duration;

const BASE_ATTACK_TIME: f32 = 2.7;

fn attack_speed(speed: f32) -> Duration {
	let s = BASE_ATTACK_TIME / (1.0 + (speed / 100.0));
	Duration::from_millis((s * 1000.0) as u64)
}

fn towers_targeting(
	mut towers: Query<(&mut Tower, &Transform), Without<TemporaryTower>>,
	slimes: Query<(Entity, &Transform), With<Slime>>,
) {
	for (mut tower, tower_transform) in towers.iter_mut() {
		tower.targets = vec![];
		for (slime_entity, slime_transform) in slimes.iter() {
			if flat_distance(*tower_transform, *slime_transform) < tower.range
				&& tower.targets.len() < tower.modifiers.get_target_count()
			{
				tower.targets.push(slime_entity);
			}
		}
	}
}

fn towers_cooldown(time: Res<Time>, mut towers: Query<&mut Tower, Without<TemporaryTower>>) {
	let time_delta = time.delta();
	for mut tower in towers.iter_mut() {
		if tower.cooldown < time_delta {
			tower.cooldown = Duration::ZERO;
		} else {
			tower.cooldown -= time_delta;
		}
	}
}

fn towers_shoot(
	mut commands: Commands,
	mut towers: Query<(&mut Tower, &Transform), Without<TemporaryTower>>,
	mut slimes: Query<(&Transform, &mut Slime)>,
	asset_server: Res<AssetServer>,
) {
	for (mut tower, transform) in towers.iter_mut() {
		if tower.cooldown == Duration::ZERO && !tower.targets.is_empty() {
			tower.cooldown = attack_speed(tower.get_attack_speed());
			for target in tower.targets.clone().iter() {
				if let Ok((slime_transform, mut slime)) = slimes.get_mut(*target) {
					let projectile_bundle = Projectile::new_bundle(
						asset_server.load("projectile.png"),
						transform,
						tower.as_ref(),
						*target,
						slime_transform.translation,
					);

					for (value, duration) in tower.modifiers.apply_armor.iter() {
						slime.modifiers.armor.insert(*value, *duration);
					}

					for (value, duration) in tower.modifiers.apply_poison.iter() {
						slime.modifiers.poison.insert(*value, *duration);
					}

					for (value, duration) in tower.modifiers.apply_speed.iter() {
						slime.modifiers.speed.insert(*value, *duration);
					}

					commands.spawn_bundle(projectile_bundle);
				}
			}
		}
	}
}
fn towers_cleave(
	towers: Query<&Tower, Without<TemporaryTower>>,
	mut slimes: Query<(&Transform, &mut Slime)>,
) {
	for tower in towers.iter() {
		for target in tower.targets.iter() {
			if tower.cooldown == Duration::ZERO {
				if let Ok((slime_transform, _)) = slimes.get(*target) {
					if let Some(cleave) = &tower.modifiers.cleave {
						for (st, mut s) in slimes.iter_mut() {
							if flat_distance(*st, *slime_transform) <= cleave.range {
								s.take_pure_damage((tower.damage * cleave.damage) as usize);
							}
						}
					}
				}
			}
		}
	}
}
pub struct TowersAIPlugin;

impl Plugin for TowersAIPlugin {
	fn build(&self, app: &mut App) {
		app.add_system_set(
			SystemSet::on_update(AppState::Enemies)
				.with_system(towers_targeting)
				.with_system(towers_cleave.label("tower_cleave"))
				.with_system(towers_shoot.after("tower_cleave"))
				.with_system(towers_cooldown),
		);
	}
}
