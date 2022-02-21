// #![allow(unused_mut, dead_code, unused_variables, unused_parens)]
use bevy::prelude::*;

mod game;
use game::*;
mod mouse;
use mouse::*;
mod pathfinding;
use pathfinding::*;
mod utils;
use utils::*;
mod enemies;
use enemies::*;
mod towers;
use towers::*;
mod towers_ai;
use towers_ai::*;
mod ui;
use ui::*;
mod projectiles;
use projectiles::*;
mod modifiers;
use modifiers::*;

const GRID_SIZE: f32 = 25.0;
const TILE_SIZE: f32 = 25.0;
const TILE_SPACER: f32 = 1.0;
const WINDOW_WIDTH: f32 = 1280.0;
const WINDOW_HEIGHT: f32 = 720.0;

fn init_cameras(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum AppState {
    Build,
    Select,
    Enemies,
}

struct MainPlugin;

impl Plugin for MainPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WindowDescriptor {
            title: "GemTD".to_string(),
            width: WINDOW_WIDTH,
            height: WINDOW_HEIGHT,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgb(0.12, 0.12, 0.12)))
        .add_state(AppState::Build)
        .add_startup_system(init_cameras)
        .add_system(bevy::input::system::exit_on_esc_system);
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(MainPlugin)
        .add_plugin(PathfindingPlugin)
        .add_plugin(MousePlugin)
        .add_plugin(GamePlugin)
        .add_plugin(EnemiesPlugin)
        .add_plugin(TowersPlugin)
        .add_plugin(TowersAIPlugin)
        .add_plugin(UiPlugin)
        .add_plugin(ProjectilesPlugin)
        .run();
}
