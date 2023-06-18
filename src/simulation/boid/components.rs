use bevy::prelude::*;

use crate::simulation::movement::components::Movement;

#[derive(Component)]
pub struct Boid {
    pub arrival_treshold: f32,
    pub separation_treshold: f32,
    pub neighbor_distance: f32,
}

impl Boid {
    pub fn seek(&self, movement: &Movement, target: Vec3) -> Vec3 {
        let desired = target - movement.position;
        let distance = desired.length();
        let desired = desired.normalize()
            * movement.max_speed
            * if distance < self.arrival_treshold {
                distance / self.arrival_treshold
            } else {
                1.
            };
        let steer = (desired - movement.velocity).clamp_length_max(movement.max_force);
        steer
    }

    pub fn align(&self, movement: &Movement, group: &Vec<Vec3>) -> Vec3 {
        let mut steer = Vec3::ZERO;
        let mut count = 0;

        for other in group {
            let distance = (movement.position - *other).length();
            if distance > 0. && distance < self.neighbor_distance {
                steer += movement.velocity;
                count += 1;
            }
        }

        if count > 0 {
            steer /= count as f32;
        }

        if steer.length() > 0. {
            steer = steer.normalize() * movement.max_speed;
            steer -= movement.velocity;
            steer = steer.clamp_length_max(movement.max_force)
        }

        steer
    }

    pub fn separate(&self, movement: &Movement, group: &Vec<Vec3>) -> Vec3 {
        let mut steer = Vec3::ZERO;
        let mut count = 0;

        for other in group {
            let distance = (movement.position - *other).length();
            if distance > 0. && distance < self.separation_treshold {
                let diff = movement.position - *other;
                steer += diff.normalize() / distance;
                count += 1;
            }
        }

        if count > 0 {
            steer /= count as f32;
        }

        if steer.length() > 0. {
            steer = steer.normalize() * movement.max_speed;
            steer -= movement.velocity;
            steer = steer.clamp_length_max(movement.max_force)
        }

        steer
    }

    pub fn cohesion(&self, movement: &Movement, group: &Vec<Vec3>) -> Vec3 {
        let mut steer = Vec3::ZERO;
        let mut count = 0;

        for other in group {
            let distance = (movement.position - *other).length();
            if distance > 0. && distance < self.neighbor_distance {
                steer += *other;
                count += 1;
            }
        }

        if count > 0 {
            steer /= count as f32;
            self.seek(movement, steer)
        } else {
            Vec3::ZERO
        }
    }
}
