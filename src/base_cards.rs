use bevy::color::palettes::css::WHITE;
use bevy::prelude::*;
use bevy::sprite::AlphaMode2d;

use crate::interaction;
use interaction::on_drag;
use interaction::Zoomable;
use interaction::{on_right_click_down, on_right_click_up};

pub fn load_base_cards(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let texture_handle = asset_server.load("bases/01-adrenaline-labs.png");
    let mesh_handle = meshes.add(Rectangle::from_size(Vec2::new(111.0, 171.0)));

    commands
        .spawn((
            Mesh2d(mesh_handle.clone()),
            MeshMaterial2d(materials.add(ColorMaterial {
                color: WHITE.into(),
                alpha_mode: AlphaMode2d::Opaque,
                texture: Some(texture_handle),
            })),
            Transform::from_xyz(-168.0, -212.0, 0.0),
            Zoomable,
        ))
        .observe(on_drag)
        .observe(on_right_click_down)
        .observe(on_right_click_up);
}
