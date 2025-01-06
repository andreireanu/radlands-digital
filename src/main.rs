use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use bevy::window::{WindowMode, WindowResolution};
use std::env;

mod background;
mod base_cards;
mod interaction;
use background::setup;
use background::{WINDOW_HEIGHT, WINDOW_WIDTH};
use base_cards::load_base_cards;

fn main() {
    env::set_var("WGPU_BACKEND", "gl");
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Radlands".to_string(), // Window title
                    mode: WindowMode::Windowed,
                    resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT)
                        .with_scale_factor_override(1.5),
                    resizable: true,
                    ..default()
                }),
                close_when_requested: true,
                exit_condition: bevy::window::ExitCondition::OnPrimaryClosed,
            }),
            MeshPickingPlugin,
            ShapePlugin,
        ))
        .add_systems(Startup, setup)
        .add_systems(Startup, load_base_cards)
        .run();
}
