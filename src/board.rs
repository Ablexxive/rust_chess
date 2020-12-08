use bevy::prelude::*;
use bevy_mod_picking::{Group, PickState, PickableMesh};

use crate::pieces::Piece;

pub struct BoardPlugin;
impl Plugin for BoardPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<SelectedSquare>()
            .init_resource::<SelectedPiece>()
            .add_startup_system(create_board.system())
            .add_system(select_square.system())
            .add_system(color_squares.system());
    }
}

pub struct Square {
    pub x: u8,
    pub y: u8,
}

impl Square {
    fn is_white(&self) -> bool {
        (self.x + self.y + 1) % 2 == 0
    }
}

#[derive(Default)]
struct SelectedSquare {
    entity: Option<Entity>,
}

#[derive(Default)]
struct SelectedPiece {
    entity: Option<Entity>,
}

fn select_square(
    mut commands: Commands,
    pick_state: Res<PickState>,
    mouse_button_inputs: Res<Input<MouseButton>>,
    mut selected_square: ResMut<SelectedSquare>,
    mut selected_piece: ResMut<SelectedPiece>,
    squares_query: Query<&Square>,
    mut pieces_query: Query<(Entity, &mut Piece)>,
) {
    if !mouse_button_inputs.just_pressed(MouseButton::Left) {
        return;
    }

    if let Some((square_entity, _intersection)) = pick_state.top(Group::default()) {
        if let Ok(square) = squares_query.get(*square_entity) {
            selected_square.entity = Some(*square_entity);

            if let Some(selected_piece_entity) = selected_piece.entity {
                let possible_enemy_pieces_vec: Vec<(Entity, Piece)> = pieces_query
                    .iter_mut()
                    .map(|(entity, piece)| (entity, *piece))
                    .collect();
                let pieces_vec = pieces_query.iter_mut().map(|(_, piece)| *piece).collect();

                // If you do find a selected piece, then update it's position.
                if let Ok((_piece_entity, mut piece)) = pieces_query.get_mut(selected_piece_entity)
                {
                    if piece.is_move_valid((square.x, square.y), pieces_vec) {
                        for (entity, other_piece) in possible_enemy_pieces_vec {
                            // If we find a piece on the target that is of the opposite color, despawn
                            // it.
                            if other_piece.x == square.x
                                && other_piece.y == square.y
                                && other_piece.color != piece.color
                            {
                                commands.despawn_recursive(entity);
                            }
                        }
                        piece.x = square.x;
                        piece.y = square.y;
                    }
                }
                selected_square.entity = None;
                selected_piece.entity = None;
            } else {
                // If there is no piece previously selected, select the current one.
                for (piece_entity, piece) in pieces_query.iter_mut() {
                    if piece.x == square.x && piece.y == square.y {
                        selected_piece.entity = Some(piece_entity);
                        break;
                    }
                }
            }
        } else {
            // Deselect everything if player clicks outside the board.
            selected_square.entity = None;
            selected_piece.entity = None;
        }
    }
}

fn color_squares(
    pick_state: Res<PickState>,
    selected_square: Res<SelectedSquare>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    query: Query<(Entity, &Square, &Handle<StandardMaterial>)>,
) {
    // Get entity under cursor
    let top_entity = if let Some((entity, _intersection)) = pick_state.top(Group::default()) {
        Some(*entity)
    } else {
        None
    };

    for (entity, square, material_handle) in query.iter() {
        let material = materials.get_mut(material_handle).unwrap();

        // Change material color
        material.albedo = if Some(entity) == top_entity {
            Color::rgb(0.8, 0.3, 0.8)
        } else if Some(entity) == selected_square.entity {
            Color::rgb(0.9, 0.1, 0.1)
        } else if square.is_white() {
            Color::rgb(1.0, 0.9, 0.9)
        } else {
            Color::rgb(0.0, 0.1, 0.1)
        };
    }
}

fn create_board(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Add meshes and materials for squares
    let square_mesh_handle = meshes.add(Mesh::from(shape::Plane { size: 1.0 }));

    // Spawn 64 squares
    for idx in 0..8 {
        for idy in 0..8 {
            commands
                .spawn(PbrComponents {
                    mesh: square_mesh_handle.clone(),
                    material: if (idx + idy + 1) % 2 == 0 {
                        materials.add(Color::rgb(1.0, 0.9, 0.9).into())
                    } else {
                        materials.add(Color::rgb(0.0, 0.1, 0.1).into())
                    },
                    transform: Transform::from_translation(Vec3::new(idx as f32, 0.0, idy as f32)),
                    ..Default::default()
                })
                .with(PickableMesh::default())
                .with(Square { x: idx, y: idy });
        }
    }
}
