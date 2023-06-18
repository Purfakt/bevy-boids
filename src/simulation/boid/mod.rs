pub mod components;
pub mod systems;

use systems::*;

use bevy::prelude::*;
pub struct BoidPlugin;

impl Plugin for BoidPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_boid)
            .add_system(update_boid_movement);
    }
}
