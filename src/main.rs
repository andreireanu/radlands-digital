use bevy::window::{WindowMode, WindowResolution};
use bevy::{prelude::*, window::PrimaryWindow};
use std::env;

pub const WINDOW_WIDTH: f32 = 4010.0;
pub const WINDOW_HEIGHT: f32 = 4010.0;

fn main() {
    env::set_var("WGPU_BACKEND", "gl");
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Radlands".to_string(), // Window title
                mode: WindowMode::Windowed,
                resolution: WindowResolution::new(2300.0, 2220.0).with_scale_factor_override(1.5),
                resizable: true,
                ..default()
            }),
            close_when_requested: true,
            exit_condition: bevy::window::ExitCondition::OnPrimaryClosed,
        }))
        .add_systems(Startup, setup)
        .add_systems(Startup, spawn_camera)
        .run();
}

pub fn setup(
    mut commands: Commands,
    _window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((Sprite::from_image(
        asset_server.load("components/board_1400.png"),
    ),));
}

pub fn spawn_camera(mut commands: Commands, _window_query: Query<&Window, With<PrimaryWindow>>) {
    commands.spawn(Camera2d {});
}
