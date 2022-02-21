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
    GameOver,
}

struct MainPlugin;

fn game_over(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    ui_root: Query<Entity, With<UiRoot>>,
    sprites: Query<Entity, With<Sprite>>,
    texture_sprites: Query<Entity, With<TextureAtlasSprite>>,
) {
    commands
        .entity(ui_root.get_single().unwrap())
        .despawn_recursive();

    for sprite in sprites.iter() {
        commands.entity(sprite).despawn_recursive();
    }
    for texture in texture_sprites.iter() {
        commands.entity(texture).despawn_recursive();
    }

    let font = asset_server.load("FiraSans-Bold.ttf");
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            color: Color::NONE.into(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                style: Style {
                    align_self: AlignSelf::FlexEnd,
                    ..Default::default()
                },
                text: Text {
                    sections: vec![
                        TextSection {
                            value: "Game Over !\n\n".to_string(),
                            style: TextStyle {
                                font: font.clone(),
                                font_size: 50.0,
                                color: Color::WHITE,
                            },
                        },
                        TextSection {
                            value: "Press esc to exit.".to_string(),
                            style: TextStyle {
                                font,
                                font_size: 30.0,
                                color: Color::WHITE,
                            },
                        },
                    ],
                    ..Default::default()
                },
                ..Default::default()
            });
        });
}

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
        .add_system_set(SystemSet::on_enter(AppState::GameOver).with_system(game_over))
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
