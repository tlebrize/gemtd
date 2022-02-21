use crate::{position_to_transform, AppState, Tower, GRID_SIZE};
use bevy::prelude::*;

const FONT_SIZE: f32 = 20.0;

#[derive(Component)]
pub struct Tooltip;

pub struct UpdateUIEvent {
	pub position: (usize, usize),
}

pub struct UpdateRangeIndicatorScaleEvent {
	pub position: (usize, usize),
	pub scale: Vec3,
}

#[derive(Component)]
struct RangeIndicator {
	position: (usize, usize),
}

fn remove_old_range_indicators() {}

fn update_range_indicator_scale(
	mut update_scale: EventReader<UpdateRangeIndicatorScaleEvent>,
	mut ranges: Query<(&RangeIndicator, &mut Transform)>,
) {
	for event in update_scale.iter() {
		for (range, mut transform) in ranges.iter_mut() {
			if range.position == event.position {
				transform.scale = event.scale;
			}
		}
	}
}

fn update_range_indicator_visibility(
	mut update_ui: EventReader<UpdateUIEvent>,
	mut ranges: Query<(&RangeIndicator, &mut Visibility)>,
) {
	for event in update_ui.iter() {
		for (indicator, mut visibility) in ranges.iter_mut() {
			visibility.is_visible = event.position == indicator.position;
		}
	}
}

fn setup_range_indicators(mut commands: Commands, asset_server: Res<AssetServer>) {
	let size = GRID_SIZE as usize;
	for y in 0..size {
		for x in 0..size {
			let transform = position_to_transform(x as f32, y as f32);
			commands
				.spawn_bundle(SpriteBundle {
					sprite: Sprite {
						color: Color::rgba(10.0, 10.0, 50.0, 0.5),
						..Default::default()
					},
					texture: asset_server.load("circle.png"),
					visibility: Visibility { is_visible: false },
					transform,
					..Default::default()
				})
				.insert(RangeIndicator { position: (x, y) });
		}
	}
}

fn update_tooltip_handler(
	mut update_ui: EventReader<UpdateUIEvent>,
	towers: Query<&Tower>,
	mut tooltips: Query<&mut Text, With<Tooltip>>,
) {
	for event in update_ui.iter() {
		for tower in towers.iter() {
			if tower.position == event.position {
				let mut text = tooltips.get_single_mut().unwrap();
				text.sections[1].value = format!("{:?}", tower.kind);
				text.sections[3].value = format!("{:?}", tower.range);
				text.sections[5].value = format!("{:?}", tower.damage);
				text.sections[7].value = format!("{:?}", tower.get_attack_speed());
				text.sections[9].value = tower.tooltip.to_string();
			}
		}
	}
}

fn setup_tooltip(mut commands: Commands, asset_server: Res<AssetServer>) {
	let font = asset_server.load("FiraSans-Bold.ttf");
	commands
		.spawn_bundle(TextBundle {
			style: Style {
				align_self: AlignSelf::FlexEnd,
				..Default::default()
			},
			text: Text {
				sections: vec![
					TextSection {
						value: "Name: ".to_string(),
						style: TextStyle {
							font: font.clone(),
							font_size: FONT_SIZE,
							color: Color::WHITE,
						},
					},
					TextSection {
						value: "".to_string(),
						style: TextStyle {
							font: font.clone(),
							font_size: FONT_SIZE,
							color: Color::GOLD,
						},
					},
					TextSection {
						value: "\nRange:".to_string(),
						style: TextStyle {
							font: font.clone(),
							font_size: FONT_SIZE,
							color: Color::WHITE,
						},
					},
					TextSection {
						value: "".to_string(),
						style: TextStyle {
							font: font.clone(),
							font_size: FONT_SIZE,
							color: Color::GOLD,
						},
					},
					TextSection {
						value: "\nDamage:".to_string(),
						style: TextStyle {
							font: font.clone(),
							font_size: FONT_SIZE,
							color: Color::WHITE,
						},
					},
					TextSection {
						value: "".to_string(),
						style: TextStyle {
							font: font.clone(),
							font_size: FONT_SIZE,
							color: Color::GOLD,
						},
					},
					TextSection {
						value: "\nSpeed:".to_string(),
						style: TextStyle {
							font: font.clone(),
							font_size: FONT_SIZE,
							color: Color::WHITE,
						},
					},
					TextSection {
						value: "".to_string(),
						style: TextStyle {
							font: font.clone(),
							font_size: FONT_SIZE,
							color: Color::GOLD,
						},
					},
					TextSection {
						value: "\nAbilities:\n".to_string(),
						style: TextStyle {
							font: font.clone(),
							font_size: FONT_SIZE,
							color: Color::WHITE,
						},
					},
					TextSection {
						value: "".to_string(),
						style: TextStyle {
							font,
							font_size: FONT_SIZE,
							color: Color::GOLD,
						},
					},
				],
				..Default::default()
			},
			..Default::default()
		})
		.insert(Tooltip);
}

pub struct UiPlugin;

impl Plugin for UiPlugin {
	fn build(&self, app: &mut App) {
		app.add_event::<UpdateUIEvent>()
			.add_event::<UpdateRangeIndicatorScaleEvent>()
			.add_startup_system(setup_tooltip)
			.add_startup_system(setup_range_indicators)
			.add_system(update_tooltip_handler)
			.add_system(update_range_indicator_visibility)
			.add_system(update_range_indicator_scale)
			.add_system_set(
				SystemSet::on_enter(AppState::Enemies).with_system(remove_old_range_indicators),
			);
	}
}
