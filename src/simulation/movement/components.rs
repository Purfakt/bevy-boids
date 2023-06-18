use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Movement {
    pub position: Vec3,
    pub velocity: Vec3,
    pub acceleration: Vec3,
    pub max_force: f32,
    pub max_speed: f32,
}

impl Movement {
    pub fn apply_force(&mut self, force: Vec3) {
        self.acceleration += force;
    }

    pub fn update(&mut self, delta_seconds: f32) {
        self.velocity += self.acceleration;
        self.velocity = self.velocity.clamp_length_max(self.max_speed);
        self.acceleration = Vec3::ZERO;

        self.position += self.velocity * delta_seconds;
    }
}
