#![allow(unused_mut, dead_code, unused_variables, unused_parens)]
use bevy::prelude::*;

mod components;
use components::*;
mod mouse;
use mouse::*;
mod pathfinding;
use pathfinding::*;

const GRID_SIZE: f32 = 16.0;
const TILE_SIZE: f32 = 40.0;
const TILE_SPACER: f32 = 2.0;
const WINDOW_WIDTH: f32 = 1280.0;
const WINDOW_HEIGHT: f32 = 720.0;

fn position_to_transform(x: f32, y: f32) -> Transform {
    Transform::from_translation(Vec3::new(
        x * TILE_SIZE - (GRID_SIZE * TILE_SIZE / 2.0) + (0.5 * TILE_SIZE) + x * TILE_SPACER
            - TILE_SPACER * 0.5,
        -1.0 * (y * TILE_SIZE - (GRID_SIZE * TILE_SIZE / 2.0)
            + (0.5 * TILE_SIZE)
            + y * TILE_SPACER
            - TILE_SPACER * 1.5),
        1.0,
    ))
}

fn vec2_to_position(vec: Vec2) -> (f32, f32) {
    let (mut x, mut y) = (
        ((vec.x + (GRID_SIZE * TILE_SIZE - TILE_SIZE + TILE_SPACER) / 2.0)
            / (TILE_SIZE + TILE_SPACER))
            .round(),
        (((vec.y * -1.0) + ((GRID_SIZE * TILE_SIZE - TILE_SIZE) / 2.0) + (TILE_SPACER * 1.5))
            / (TILE_SIZE + TILE_SPACER))
            .round(),
    );
    if x == -0.0 {
        x = 0.0
    }
    if y == -0.0 {
        y = 0.0
    }
    (x, y)
}

fn fit_to_grid(pos: (f32, f32)) -> Option<(usize, usize)> {
    if 0.0 > pos.0 || 0.0 > pos.1 || GRID_SIZE < pos.0 || GRID_SIZE < pos.1 {
        None
    } else {
        Some((pos.0 as usize, pos.1 as usize))
    }
}

fn init_game(mut commands: Commands, mut game: ResMut<Game>, mut graph: ResMut<Graph>) {
    let size = GRID_SIZE as usize;
    for y in 0..size {
        game.grid.push(vec![]);
        for x in 0..size {
            let (content, color, walkable) = if x == 0 || y == 0 || x == size - 1 || y == size - 1 {
                (CellContent::Limit, Color::WHITE, false)
            } else if x == 1 && y == 1 {
                (CellContent::Start, Color::GREEN, true)
            } else if x == size - 2 && y == size - 2 {
                (CellContent::End, Color::RED, true)
            } else {
                (CellContent::Empty, Color::BLACK, true)
            };

            let node_id = graph.add(walkable, x, y);

            let cell = Cell {
                content: content.clone(),
                position: [x, y],
                node_id,
            };

            let entity = commands
                .spawn_bundle(SpriteBundle {
                    sprite: Sprite {
                        color,
                        custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                        ..Default::default()
                    },
                    transform: position_to_transform(x as f32, y as f32),
                    ..Default::default()
                })
                .insert(cell)
                .id();
            game.grid[y].push(entity);

            match content {
                CellContent::Start => graph.start = node_id,
                CellContent::End => graph.end = node_id,
                _ => {}
            }
        }
    }
}

fn update_cell_sprites(mut commands: Commands, mut query: Query<(&Cell, &mut Sprite)>) {
    for (cell, mut sprite) in query.iter_mut() {
        match cell.content {
            CellContent::Empty => {}
            CellContent::Limit => sprite.color = Color::WHITE,
            CellContent::Start => sprite.color = Color::GREEN,
            CellContent::End => sprite.color = Color::RED,
            CellContent::Rock => sprite.color = Color::GRAY,
        }
    }
}

fn init_cameras(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
}

fn init_mouse(mut windows: ResMut<Windows>) {
    windows
        .get_primary_mut()
        .unwrap()
        .set_cursor_lock_mode(true);
}

struct NewPathEvent(Vec<NodeId>);

fn handle_click(
    mut commands: Commands,
    mut mouse: ResMut<MouseState>,
    game: Res<Game>,
    mut graph: ResMut<Graph>,
    mut cells: Query<(&mut Cell,)>,
    mut new_path: EventWriter<NewPathEvent>,
) {
    if mouse.pressed && !mouse.pressed_read {
        mouse.pressed_read = true;
        if let Some((x, y)) = fit_to_grid(vec2_to_position(mouse.position)) {
            let entity = game.grid[y][x];
            if let Ok((mut cell,)) = cells.get_mut(entity) {
                if cell.content == CellContent::Empty {
                    cell.content = CellContent::Rock;
                    graph.set_node_walkability(cell.node_id, false);
                } else {
                    return;
                }
            }
            if let Some(path) = graph.bfs() {
                println!("werks: {:?}", path);
                new_path.send(NewPathEvent(path));
            } else {
                let (mut cell,) = cells.get_mut(entity).unwrap();
                cell.content = CellContent::Empty;
                graph.set_node_walkability(cell.node_id, true);
            }
        }
    }
}

fn update_path(
    mut commands: Commands,
    mut new_path: EventReader<NewPathEvent>,
    graph: Res<Graph>,
    game: Res<Game>,
    mut query: Query<&mut Sprite>,
) {
    for event in new_path.iter() {
        for (mut sprite) in query.iter_mut() {
            if sprite.color == Color::PURPLE {
                sprite.color = Color::BLACK;
            }
        }
        for node_id in event.0.iter() {
            let (x, y) = graph.get_node_coordinates(*node_id).unwrap();
            let mut sprite = query.get_mut(game.grid[y][x]).unwrap();
            sprite.color = Color::PURPLE;
        }
    }
}

#[derive(Component)]
struct Debugger;

fn debug_mouse(
    mut commands: Commands,
    mouse: Res<MouseState>,
    mut debugger: Query<(&mut Transform, &mut Sprite), With<Debugger>>,
) {
    if debugger.is_empty() {
        commands
            .spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    color: Color::YELLOW,
                    custom_size: Some(Vec2::new(100.0, 100.0)),
                    ..Default::default()
                },
                transform: Transform::from_translation(Vec3::new(
                    mouse.position.x,
                    mouse.position.y,
                    0.0,
                )),
                ..Default::default()
            })
            .insert(Debugger);
        return;
    }

    let (mut transform, mut sprite) = debugger.get_single_mut().unwrap();

    sprite.color = match mouse.pressed {
        true => Color::YELLOW,
        false => Color::PINK,
    };

    transform.translation.x = mouse.position.x;
    transform.translation.y = mouse.position.y;
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(WindowDescriptor {
            title: "Wordle".to_string(),
            width: WINDOW_WIDTH,
            height: WINDOW_HEIGHT,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgb(0.12, 0.12, 0.12)))
        .insert_resource(Game { grid: vec![] })
        .insert_resource(Graph::default())
        .insert_resource(MouseState::new(WINDOW_WIDTH, WINDOW_HEIGHT))
        .add_startup_system(init_cameras)
        .add_startup_system(init_game)
        .add_startup_system(init_mouse)
        .add_event::<NewPathEvent>()
        .add_system(handle_click)
        .add_system(handle_mouse_events)
        .add_system(debug_mouse)
        .add_system(update_cell_sprites)
        .add_system(update_path)
        .add_system(bevy::input::system::exit_on_esc_system)
        .run();
}
