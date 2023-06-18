use bevy::{prelude::*, window::PrimaryWindow};

use bevy_boids::simulation::SimulationPlugin;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (1000., 1000.).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugin(SimulationPlugin)
        .add_startup_system(spawn_camera)
        .run();
}

#[derive(Component)]
pub struct MainCamera;

fn window_center_transform(window_query: Query<&Window, With<PrimaryWindow>>) -> Transform {
    let window = window_query.get_single().unwrap();
    Transform {
        translation: Vec3::new(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    }
}

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    commands.spawn((
        MainCamera {},
        Camera2dBundle {
            transform: window_center_transform(window_query),
            ..default()
        },
    ));
}
