use bevy::prelude::*;
use bevy_mod_picking::PickableMesh;

pub fn create_board(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Add meshes and materials for squares
    let square_mesh_handle = meshes.add(Mesh::from(shape::Plane { size: 1.0 }));
    let white_material_handle = materials.add(Color::rgb(1.0, 0.9, 0.9).into());
    let black_material_handle = materials.add(Color::rgb(0.0, 0.1, 0.1).into());

    // Spawn 64 squares
    for idx in 0..8 {
        for idy in 0..8 {
            commands
                .spawn(PbrComponents {
                    mesh: square_mesh_handle.clone(),
                    material: if (idx + idy + 1) % 2 == 0 {
                        white_material_handle.clone()
                    } else {
                        black_material_handle.clone()
                    },
                    transform: Transform::from_translation(Vec3::new(idx as f32, 0.0, idy as f32)),
                    ..Default::default()
                })
                .with(PickableMesh::default());
        }
    }
}
