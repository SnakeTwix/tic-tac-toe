use bevy::prelude::*;
use bevy_mod_picking::PickingEvent;
use iyes_loopless::prelude::*;

use crate::{playing, util::marker_to_num, Turn};
pub struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_system(mark_cell.run_if(playing))
            .add_system(place_marker.run_if(playing));
    }
}

#[derive(Clone, Copy)]
pub enum MarkerType {
    X,
    O,
}

#[derive(Resource)]
pub struct Grid {
    pub state: Vec<i32>,
}

impl Default for Grid {
    fn default() -> Self {
        Self { state: vec![0; 9] }
    }
}

#[derive(Component)]
pub struct Cell {
    pub marker: Option<MarkerType>,
    pub index: usize,
}

impl Cell {
    pub fn new(index: usize) -> Self {
        Self {
            marker: None,
            index,
        }
    }

    pub fn mark(&mut self, marker: MarkerType) {
        self.marker = Some(marker);
    }
}

#[derive(Resource, Debug)]
pub struct MarkerAssets {
    x: Handle<Image>,
    o: Handle<Image>,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let x: Handle<Image> = asset_server.load("x.png");
    let o: Handle<Image> = asset_server.load("o.png");

    commands.insert_resource(MarkerAssets { x, o });
    commands.insert_resource(Grid::default());
}

fn place_marker(
    query: Query<(Entity, &Cell), Changed<Cell>>,
    mut commands: Commands,
    markers: Res<MarkerAssets>,
) {
    for (cell_id, cell) in query.iter() {
        commands.entity(cell_id).with_children(|parent| {
            let texture = match cell.marker {
                Some(MarkerType::X) => markers.x.clone(),
                Some(MarkerType::O) => markers.o.clone(),
                None => return,
            };

            parent.spawn(SpriteBundle {
                transform: Transform::from_xyz(0., 0., 1.),
                sprite: Sprite {
                    custom_size: Some(Vec2::from_array([200., 200.])),
                    ..default()
                },

                texture,

                ..default()
            });
        });
    }
}

fn mark_cell(
    mut query: Query<&mut Cell>,
    mut events: EventReader<PickingEvent>,
    mut turn: ResMut<Turn>,
    mut grid: ResMut<Grid>,
) {
    for event in events.iter() {
        match event {
            PickingEvent::Clicked(e) => {
                let mut cell = query.get_mut(*e).unwrap();

                if cell.marker.is_none() {
                    let marker = turn.0;
                    turn.toggle();

                    cell.mark(marker);

                    let index = cell.index;
                    grid.state[index] = marker_to_num(&marker);
                }
            }

            _ => (),
        }
    }
}
