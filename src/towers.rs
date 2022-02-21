use crate::{
	flat_distance, position_to_transform, AppState, Cell, CellContent, Cleave, RockPlacedEvent,
	TowerAuras, TowerModifier, UpdateRangeIndicatorScaleEvent, UpdateTowerTooltipEvent,
};
use bevy::prelude::*;
use bevy::utils::Duration;
use rand::Rng;

const RANGE_SCALE: f32 = 4.0;

fn scale_range(range: f32) -> f32 {
	range / RANGE_SCALE
}

#[derive(std::cmp::PartialEq, Clone, Debug)]
pub enum TowerKind {
	Amethyst,
	Aquamarine,
	Diamond,
	Emerald,
	Opal,
	Ruby,
	Sapphire,
	Topaz,
}

impl TowerKind {
	fn from_usize(value: usize) -> Self {
		match value {
			0 => Self::Diamond,
			1 => Self::Ruby,
			2 => Self::Emerald,
			3 => Self::Topaz,
			4 => Self::Amethyst,
			5 => Self::Sapphire,
			6 => Self::Opal,
			7 => Self::Aquamarine,
			_ => panic!("Not a tower kind."),
		}
	}
}

#[derive(std::cmp::PartialEq, Clone, Component)]
pub struct Tower {
	pub kind: TowerKind,
	pub cell: Entity,
	pub position: (usize, usize),
	pub range: f32,
	pub cooldown: Duration,
	pub attack_speed: f32,
	pub targets: Vec<Entity>,
	pub damage: f32,
	pub tooltip: String,
	pub modifiers: TowerModifier,
	pub auras: TowerAuras,
	pub recieved_auras: TowerModifier,
}

impl Tower {
	pub fn get_attack_speed(&self) -> f32 {
		let mut attack_speed = self.attack_speed;
		for modifier in self.modifiers.attack_speed.iter() {
			attack_speed += modifier;
		}
		for modifiers in self.recieved_auras.attack_speed.iter() {
			attack_speed += modifiers;
		}
		attack_speed
	}

	fn new(kind: TowerKind, cell: Entity, position: (usize, usize)) -> Self {
		let mut tower = Self {
			kind,
			cell,
			position,
			range: 0.0,
			cooldown: Duration::from_secs(0),
			attack_speed: 0.0,
			targets: vec![],
			damage: 0.0,
			modifiers: TowerModifier::default(),
			auras: TowerAuras::default(),
			recieved_auras: TowerModifier::default(),
			tooltip: String::new(),
		};
		match tower.kind {
			TowerKind::Amethyst => {
				tower.range = scale_range(500.0);
				tower.damage = 2.0;
				tower.attack_speed = 283.0;
				tower
					.modifiers
					.apply_armor
					.insert(-2, Duration::from_secs(5));
				tower.tooltip = "Decrease enemy's armor by 2.".to_string();
			}
			TowerKind::Aquamarine => {
				tower.range = scale_range(400.0);
				tower.damage = 2.0;
				tower.attack_speed = 367.0;
				tower.modifiers.attack_speed.push(200.0);
				tower.tooltip = "+ 200 attack speed.".to_string();
			}
			TowerKind::Diamond => {
				tower.range = scale_range(500.0);
				tower.damage = 5.0;
				tower.attack_speed = 170.0;
			}
			TowerKind::Emerald => {
				tower.range = scale_range(500.0);
				tower.damage = 2.0;
				tower.attack_speed = 170.0;
				tower
					.modifiers
					.apply_poison
					.insert(2, Duration::from_secs(5));
				tower.tooltip = "Enemies take 2 damage per second.\nLasts 5 seconds.".to_string();
			}
			TowerKind::Opal => {
				tower.range = scale_range(500.0);
				tower.damage = 1.0;
				tower.attack_speed = 170.0;
				tower.auras.attack_speed.push((20.0, scale_range(500.0)));
				tower.tooltip = "Increases allies attack speed by 20.".to_string();
			}
			TowerKind::Ruby => {
				tower.range = scale_range(500.0);
				tower.damage = 4.0;
				tower.attack_speed = 170.0;
				tower.modifiers.cleave = Some(Cleave {
					range: scale_range(300.0),
					damage: 0.3,
				})
			}
			TowerKind::Sapphire => {
				tower.range = scale_range(600.0);
				tower.damage = 2.0;
				tower.attack_speed = 170.0;
				tower
					.modifiers
					.apply_speed
					.insert(30, Duration::from_secs(5));
				tower.tooltip = "Decreases enemy's movement speed by 30%.".to_string();
			}
			TowerKind::Topaz => {
				tower.range = scale_range(600.0);
				tower.damage = 3.0;
				tower.attack_speed = 131.0;
				tower.modifiers.target_count = Some(3);
				tower.tooltip = "Attacks up to 3 enemy's at the same time.".to_string();
			}
		};
		tower
	}
}

#[derive(Component)]
pub struct TemporaryTower {
	pub ui: Entity,
}

pub struct TowersPlugin;

#[derive(Default)]
struct TowerAtlasHandle {
	pub handle: Option<Handle<TextureAtlas>>,
}

fn spawn_tower_event_handler(
	mut commands: Commands,
	mut rock_placed: EventReader<RockPlacedEvent>,
	mut update_range_scale: EventWriter<UpdateRangeIndicatorScaleEvent>,
	towers_atlas_handle: Res<TowerAtlasHandle>,
	asset_server: Res<AssetServer>,
	mut cells: Query<&mut Cell>,
) {
	if let Some(texture_atlas) = &towers_atlas_handle.handle {
		let mut rng = rand::thread_rng();

		for rock in rock_placed.iter() {
			let mut cell = cells.get_mut(rock.entity).unwrap();
			let index = rng.gen_range(0..8);
			let kind = TowerKind::from_usize(index);

			let transform = position_to_transform(cell.position.0 as f32, cell.position.1 as f32);
			let tower = Tower::new(kind, rock.entity, cell.position);

			let temporary_tower_ui = commands
				.spawn_bundle(SpriteBundle {
					sprite: Sprite {
						color: Color::rgb(0.9, 0.1, 0.1),
						custom_size: Some(Vec2::new(30.0, 30.0)),
						..Default::default()
					},
					transform,
					texture: asset_server.load("thick-circle.png"),
					..Default::default()
				})
				.id();

			let tower_id = commands
				.spawn_bundle(SpriteSheetBundle {
					texture_atlas: texture_atlas.clone(),
					transform: transform.with_scale(Vec3::new(2.0, 2.0, 1.0)),
					sprite: TextureAtlasSprite {
						index,
						..Default::default()
					},
					..Default::default()
				})
				.insert(tower.clone())
				.insert(TemporaryTower {
					ui: temporary_tower_ui,
				})
				.id();

			cell.content = CellContent::Tower(tower_id);

			update_range_scale.send(UpdateRangeIndicatorScaleEvent {
				position: cell.position,
				scale: Vec3::new(tower.range / 50.0, tower.range / 50.0, 1.0),
			})
		}
	}
}

fn init_spritesheet(
	asset_server: Res<AssetServer>,
	mut texture_atlases: ResMut<Assets<TextureAtlas>>,
	mut towers_atlas_handle: ResMut<TowerAtlasHandle>,
) {
	let texture_handle = asset_server.load("gems-spritesheet.png");
	let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(16.0, 16.0), 8, 9);
	towers_atlas_handle.handle = Some(texture_atlases.add(texture_atlas));
}

fn clear_auras(mut query: Query<&mut Tower>) {
	for mut tower in query.iter_mut() {
		tower.recieved_auras = TowerModifier::default();
	}
}

fn set_auras(
	mut query: Query<(&mut Tower, &Transform)>,
	mut update_ui: EventWriter<UpdateTowerTooltipEvent>,
) {
	let mut combinations = query.iter_combinations_mut();
	while let Some([(mut a, at), (mut b, bt)]) = combinations.fetch_next() {
		// TODO refactor this with a function
		for (value, range) in a.auras.attack_speed.iter() {
			if flat_distance(*at, *bt) <= *range {
				b.recieved_auras.attack_speed.push(*value);
				update_ui.send(UpdateTowerTooltipEvent {
					position: b.position,
				});
			}
		}
		for (value, range) in b.auras.attack_speed.iter() {
			if flat_distance(*bt, *at) <= *range {
				a.recieved_auras.attack_speed.push(*value);
				update_ui.send(UpdateTowerTooltipEvent {
					position: a.position,
				});
			}
		}
	}
}

impl Plugin for TowersPlugin {
	fn build(&self, app: &mut App) {
		app.insert_resource(TowerAtlasHandle::default())
			.add_startup_system(init_spritesheet)
			.add_system_set(
				SystemSet::on_update(AppState::Build).with_system(spawn_tower_event_handler),
			)
			.add_system_set(
				SystemSet::on_enter(AppState::Enemies)
					.with_system(clear_auras.before("set_auras"))
					.with_system(set_auras.label("set_auras")),
			);
	}
}
