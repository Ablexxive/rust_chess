// From: https://caballerocoll.com/blog/bevy-chess-tutorial/
use bevy::prelude::*;

fn main() {
    App::build()
        .add_resource(Msaa { samples: 4 })
        .add_resource(WindowDescriptor {
            title: "Chess :D".to_string(),
            width: 1600,
            height: 1600,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_startup_system(create_board.system())
        .run();
}

fn setup(mut commands: Commands) {
    commands
        // Camera
        .spawn(Camera3dComponents {
            transform: Transform::from_matrix(Mat4::from_rotation_translation(
                Quat::from_xyzw(-0.3, -0.5, -0.3, 0.5).normalize(),
                Vec3::new(-7.0, 20.0, 4.0),
            )),
            ..Default::default()
        })
        // Light
        .spawn(LightComponents {
            transform: Transform::from_translation(Vec3::new(4.0, 8.0, 4.0)),
            ..Default::default()
        });
}

fn create_board(
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
            commands.spawn(PbrComponents {
                mesh: square_mesh_handle.clone(),
                material: if (idx + idy + 1) % 2 == 0 {
                    white_material_handle.clone()
                } else {
                    black_material_handle.clone()
                },
                transform: Transform::from_translation(Vec3::new(idx as f32, 0.0, idy as f32)),
                ..Default::default()
            });
        }
    }
}
