#![allow(unused_variables)]
use bevy::{
    core_pipeline::clear_color::ClearColorConfig, prelude::*, sprite::MaterialMesh2dBundle,
};
use bevy_mod_picking::{
    DebugCursorPickingPlugin, InteractablePickingPlugin, PickableBundle, PickingCameraBundle,
    PickingPlugin,
};
use grid::{Cell, Grid, GridPlugin, MarkerType};
use iyes_loopless::prelude::*;

mod grid;
mod util;

fn main() {
    App::new()
        .insert_resource(GameState::Playing)
        .add_plugins(DefaultPlugins)
        .add_plugin(PickingPlugin)
        .add_plugin(InteractablePickingPlugin)
        // .add_plugin(DebugEventsPickingPlugin)
        .add_plugin(DebugCursorPickingPlugin)
        .add_plugin(GridPlugin)
        .add_startup_system(setup)
        .add_system(check_game_state.run_if(playing))
        .run();
}

#[derive(Resource, PartialEq, Eq)]
enum GameState {
    Playing,
    Over,
}

#[derive(Resource)]
struct Turn(MarkerType);

impl Turn {
    pub fn toggle(&mut self) {
        self.0 = match self.0 {
            MarkerType::O => MarkerType::X,
            MarkerType::X => MarkerType::O,
        }
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.insert_resource(Turn(MarkerType::X));

    commands.spawn((
        Camera2dBundle {
            camera_2d: Camera2d {
                clear_color: ClearColorConfig::Custom(Color::BLACK),
            },
            ..default()
        },
        PickingCameraBundle::default(),
    ));

    let cell_size = 200.;
    let grid_offset = -cell_size;
    let cell_distance = 10.;
    commands
        .spawn(SpatialBundle {
            transform: Transform::from_xyz(
                grid_offset - cell_distance,
                grid_offset - cell_distance,
                1.0,
            ),
            ..default()
        })
        .with_children(|parent| {
            for i in 0..3 {
                for j in 0..3 {
                    parent.spawn((
                        Cell::new(i * 3 + j),
                        MaterialMesh2dBundle {
                            transform: Transform::from_xyz(
                                (cell_size + cell_distance) * i as f32,
                                (cell_size + cell_distance) * j as f32,
                                0.0,
                            ),
                            mesh: meshes
                                .add(Mesh::from(shape::Quad {
                                    size: Vec2::from_array([cell_size, cell_size]),
                                    ..default()
                                }))
                                .into(),
                            material: materials.add(ColorMaterial::from(Color::Hsla {
                                hue: 281.,
                                saturation: 0.41,
                                lightness: 0.88,
                                alpha: 1.0,
                            })),
                            ..default()
                        },
                        PickableBundle::default(),
                    ));
                }
            }
        });
}

fn check_game_state(grid: Res<Grid>, mut state: ResMut<GameState>) {
    /*
        0 1 2
        3 4 5
        6 7 8
    */
    let solutions = [
        // Horizontal
        [0, 1, 2],
        [3, 4, 5],
        [6, 7, 8],
        // Vertical
        [0, 3, 6],
        [1, 4, 7],
        [2, 5, 8],
        // Diagonal
        [0, 4, 8],
        [2, 4, 6],
    ];

    let o_sols: Vec<i32> = solutions
        .iter()
        .map(|solution| {
            solution.iter().fold(
                0,
                |acc, &val| if grid.state[val] == 2 { acc + 1 } else { acc },
            )
        })
        .collect();

    let x_sols: Vec<i32> = solutions
        .iter()
        .map(|solution| {
            solution.iter().fold(
                0,
                |acc, &val| if grid.state[val] == 1 { acc + 1 } else { acc },
            )
        })
        .collect();
    let empty_spaces = grid.state.iter().filter(|&&val| val == 0).count();

    if o_sols.contains(&3) {
        println!("O won");
        *state = GameState::Over;
    } else if x_sols.contains(&3) {
        println!("X won");
        *state = GameState::Over;
    } else if empty_spaces == 0 {
        println!("Draw");
        *state = GameState::Over;
    }
}

fn playing(state: Res<GameState>) -> bool {
    *state == GameState::Playing
}
