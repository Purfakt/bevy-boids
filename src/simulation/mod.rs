pub mod boid;
pub mod movement;

use bevy::prelude::*;

pub struct SimulationPlugin;

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(boid::BoidPlugin)
            .add_plugin(movement::MovementPlugin);
    }
}
