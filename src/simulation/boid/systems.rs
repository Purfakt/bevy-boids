use bevy::{
    prelude::*,
    render::render_resource::PrimitiveTopology,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

use super::components::Boid;
use crate::simulation::movement::components::Movement;

pub fn spawn_boid(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut boid_mesh = Mesh::new(PrimitiveTopology::LineStrip);

    let vertices = vec![
        [0., 10., 0.],
        [10., -10., 0.],
        [0., -4., 0.],
        [-10., -10., 0.],
        [0., 10., 0.],
    ];

    boid_mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);

    for _ in 0..100 {
        let position = Vec3::new(
            rand::random::<f32>() * 800.,
            rand::random::<f32>() * 600.,
            0.0,
        );

        let scale_factor = 0.5 + rand::random::<f32>();
        let a = 50.;
        let b = 10.;
        let c = 5.;

        let arrival_treshold = b + scale_factor * b;
        let separation_treshold = a + scale_factor * a;
        let neighbor_distance = a + scale_factor * a;

        let max_force = c + scale_factor * c / 5.;
        let max_speed = 2. * a + scale_factor * a;

        let color = Color::rgb(scale_factor - 0.5, scale_factor - 0.2, 0.3);

        commands.spawn((
            Boid {
                arrival_treshold,
                separation_treshold,
                neighbor_distance,
            },
            Movement {
                position,
                max_force,
                max_speed,
                ..default()
            },
            MaterialMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(boid_mesh.clone())),
                material: materials.add(ColorMaterial::from(color)),
                transform: Transform {
                    translation: position,
                    ..default()
                },
                ..default()
            },
        ));
    }
}

pub fn update_boid_movement(mut movement_query: Query<(&mut Movement, &Boid)>, time: Res<Time>) {
    let movement_ro_query = movement_query
        .iter()
        .map(|(movement, _)| movement.position)
        .collect::<Vec<Vec3>>();

    for (mut movement, boid) in movement_query.iter_mut() {
        let separate = boid.separate(&movement, movement_ro_query.as_ref()) * 1.5;
        let align = boid.align(&movement, movement_ro_query.as_ref()) * 1.;
        let cohesion = boid.cohesion(&movement, movement_ro_query.as_ref()) * 1.;
        movement.apply_force(separate + align + cohesion);
        movement.update(time.delta_seconds());
    }
}
