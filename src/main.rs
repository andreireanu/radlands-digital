use bevy::input::common_conditions::*;
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
use interaction::{on_scroll_click_down, on_scroll_click_up, scroll_events};

fn main() {
    env::set_var("WGPU_BACKEND", "vulkan");
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Radlands".to_string(), // Window title
                    mode: WindowMode::Windowed,
                    resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT),
                    position: WindowPosition::new(IVec2::new(0, 0)),
                    resizable: false,
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
        .add_systems(
            Update,
            on_scroll_click_down.run_if(input_just_pressed(MouseButton::Middle)),
        )
        .add_systems(
            Update,
            on_scroll_click_up.run_if(input_just_released(MouseButton::Middle)),
        )
        .add_systems(Update, scroll_events)
        .run();
}
