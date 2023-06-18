use bevy::{prelude::*, window::PrimaryWindow};

use super::components::Movement;

pub fn update_position(
    mut boid_query: Query<(&mut Transform, &mut Movement)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();
    let window_width = window.width();
    let window_height = window.height();

    for (mut transform, mut movement) in boid_query.iter_mut() {
        if movement.position.x > window_width {
            movement.position.x = 0.;
        } else if movement.position.x < 0. {
            movement.position.x = window_width;
        }

        if movement.position.y > window_height {
            movement.position.y = 0.;
        } else if movement.position.y < 0. {
            movement.position.y = window_height;
        }

        transform.translation = movement.position;
    }
}

pub fn update_rotation(mut boid_query: Query<(&mut Transform, &Movement)>) {
    for (mut transform, movement) in boid_query.iter_mut() {
        let sign = -movement.velocity.signum().x;
        let angle = movement.velocity.angle_between(Vec3::Y);
        transform.rotation = Quat::from_rotation_z(angle * sign);
    }
}
