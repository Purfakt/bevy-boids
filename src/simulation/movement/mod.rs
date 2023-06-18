pub mod components;
pub mod systems;

use systems::*;

use bevy::prelude::*;

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(update_position).add_system(update_rotation);
    }
}
