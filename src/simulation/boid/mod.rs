pub mod components;
pub mod systems;

use systems::*;

use bevy::prelude::*;
pub struct BoidPlugin;

impl Plugin for BoidPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_boid)
            .add_systems(Update, update_boid_movement);
    }
}
