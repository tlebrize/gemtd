#![allow(unused_mut, dead_code, unused_variables)]

use bevy::input::mouse::MouseButtonInput;
use bevy::input::ElementState;
use bevy::prelude::*;

pub struct MouseState {
	pub position: Vec2,
	pub pressed: bool,
	pub pressed_read: bool,
	width: f32,
	height: f32,
}

impl MouseState {
	pub fn new(width: f32, height: f32) -> Self {
		MouseState {
			position: Vec2::new(0.0, 0.0),
			pressed: false,
			pressed_read: true,
			width,
			height,
		}
	}

	pub fn update(&mut self, position: Vec2) {
		self.position.x = position.x - (self.width / 2.0);
		self.position.y = position.y - (self.height / 2.0);
	}
}

pub fn handle_mouse_events(
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
