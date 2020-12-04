use bevy::prelude::*;
use bevy_mod_picking::{Group, PickState, PickableMesh};

pub struct BoardPlugin;
impl Plugin for BoardPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<SelectedSquare>()
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

fn select_square(
    pick_state: Res<PickState>,
    mouse_button_inputs: Res<Input<MouseButton>>,
    mut selected_square: ResMut<SelectedSquare>,
) {
    if !mouse_button_inputs.just_pressed(MouseButton::Left) {
        return;
    }

    selected_square.entity = if let Some((entity, _intersection)) = pick_state.top(Group::default())
    {
        Some(*entity)
    } else {
        None
    };
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
