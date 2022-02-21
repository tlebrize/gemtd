use crate::{Slime, Tower, TowerKind};

use bevy::prelude::*;
use bevy::utils::Duration;

#[derive(Component)]
pub struct Projectile {
	target_position: Vec3,
	target_enemy: Option<Entity>,
	damage: f32,
	lifetime: Duration,
	velocity: Vec3,
}

#[derive(Bundle)]
pub struct ProjectileBundle {
	projectile: Projectile,
	#[bundle]
	sprite_bundle: SpriteBundle,
}

impl Projectile {
	pub fn new_bundle(
		texture: Handle<Image>,
		origin: &Transform,
		tower: &Tower,
		target: Entity,
		target_position: Vec3,
	) -> ProjectileBundle {
		let velocity = (target_position - origin.translation).normalize();

		let color = match tower.kind {
			TowerKind::Amethyst => Color::FUCHSIA,
			TowerKind::Aquamarine => Color::AQUAMARINE,
			TowerKind::Diamond => Color::SILVER,
			TowerKind::Emerald => Color::DARK_GREEN,
			TowerKind::Opal => Color::ANTIQUE_WHITE,
			TowerKind::Ruby => Color::TOMATO,
			TowerKind::Sapphire => Color::ALICE_BLUE,
			TowerKind::Topaz => Color::GOLD,
		};

		ProjectileBundle {
			projectile: Projectile {
				target_enemy: Some(target),
				lifetime: Duration::from_millis(500),
				damage: tower.damage,
				target_position,
				velocity,
			},
			sprite_bundle: SpriteBundle {
				sprite: Sprite {
					color,
					..Default::default()
				},
				transform: *origin,
				texture,
				..Default::default()
			},
		}
	}
}

fn update_projectiles(
	mut commands: Commands,
	time: Res<Time>,
	mut projectiles: Query<(Entity, &mut Projectile, &mut Transform)>,
	mut slimes: Query<&mut Slime>,
) {
	for (entity, mut projectile, mut transform) in projectiles.iter_mut() {
		if projectile.target_enemy.is_none() {
			commands.entity(entity).despawn_recursive();
			return;
		}
		if projectile.lifetime <= time.delta() {
			projectile.lifetime = Duration::from_secs(0);
		} else {
			projectile.lifetime -= time.delta();
		}

		if projectile
			.target_position
			.abs_diff_eq(transform.translation, 10.0)
			|| projectile.lifetime <= Duration::ZERO
		{
			commands.entity(entity).despawn_recursive();
			if let Ok(mut slime) = slimes.get_mut(projectile.target_enemy.unwrap()) {
				slime.take_physical_damage(projectile.damage);
			}
		}

		transform.translation.x += projectile.velocity.x * time.delta_seconds() * 1000.0;
		transform.translation.y += projectile.velocity.y * time.delta_seconds() * 1000.0;
	}
}

pub struct ProjectilesPlugin;

impl Plugin for ProjectilesPlugin {
	fn build(&self, app: &mut App) {
		app.add_system(update_projectiles);
	}
}
