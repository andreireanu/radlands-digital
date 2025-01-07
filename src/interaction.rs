use bevy::input::mouse::MouseWheel;
use bevy::picking::events::Drag;
use bevy::prelude::*;

pub const RIGHT_CLICK_ZOOM_FACTOR: f32 = 2.0;
pub const SCROLL_ZOOM_FACTOR: f32 = 0.80;
pub const MIN_SCRLLL_ZOOM: f32 = 0.5;
pub const MAX_SCROLL_ZOOM: f32 = 1.;
pub const DRAG_SENSITIVITY: f32 = 1.;

pub fn on_left_mouse_drag(
    drag: Trigger<Pointer<Drag>>,
    mut transforms: Query<&mut Transform, Without<Camera2d>>,
    camera_transform: Query<&Transform, With<Camera2d>>,
) {
    if let Ok(mut transform) = transforms.get_mut(drag.entity()) {
        if let Ok(camera_transform) = camera_transform.get_single() {
            if drag.event().button == PointerButton::Primary {
                on_drag(drag, &mut transform, camera_transform);
            }
        }
    }
}

pub fn on_right_click_down(click: Trigger<Pointer<Down>>, mut transforms: Query<&mut Transform>) {
    if let Ok(mut transform) = transforms.get_mut(click.entity()) {
        if click.event().button == PointerButton::Secondary {
            transform.scale = Vec3::splat(RIGHT_CLICK_ZOOM_FACTOR);
        }
    }
}

pub fn on_right_click_up(click: Trigger<Pointer<Up>>, mut transforms: Query<&mut Transform>) {
    if let Ok(mut transform) = transforms.get_mut(click.entity()) {
        if click.event().button == PointerButton::Secondary {
            transform.scale = Vec3::splat(1.0);
        }
    }
}

pub fn on_scroll_click_down(mut query: Query<&mut Transform, With<Camera2d>>) {
    if let Ok(mut transform) = query.get_single_mut() {
        transform.scale *= SCROLL_ZOOM_FACTOR;
    }
}

pub fn on_scroll_click_up(mut query: Query<&mut Transform, With<Camera2d>>) {
    if let Ok(mut transform) = query.get_single_mut() {
        transform.scale *= 1. / SCROLL_ZOOM_FACTOR;
    }
}

pub fn scroll_events(
    mut evr_scroll: EventReader<MouseWheel>,
    mut transform: Query<&mut Transform, With<Camera2d>>,
) {
    use bevy::input::mouse::MouseScrollUnit;

    for ev in evr_scroll.read() {
        if let Ok(mut transform) = transform.get_single_mut() {
            match ev.unit {
                MouseScrollUnit::Line => match ev.y > 0.0 {
                    true => {
                        transform.scale *= SCROLL_ZOOM_FACTOR;
                    }
                    false => transform.scale *= 1. / SCROLL_ZOOM_FACTOR,
                },
                MouseScrollUnit::Pixel => match ev.y > 0.0 {
                    true => transform.scale *= SCROLL_ZOOM_FACTOR,
                    false => transform.scale *= 1. / SCROLL_ZOOM_FACTOR,
                },
            }
            transform.scale = transform
                .scale
                .clamp(Vec3::splat(MIN_SCRLLL_ZOOM), Vec3::splat(MAX_SCROLL_ZOOM));
        }
    }
}

// Internal functions

fn on_drag(drag: Trigger<Pointer<Drag>>, transform: &mut Transform, camera_transform: &Transform) {
    println!("{:?}", camera_transform);
    let delta_x = drag.delta.x * DRAG_SENSITIVITY * camera_transform.scale.x;
    let delta_y = drag.delta.y * DRAG_SENSITIVITY * camera_transform.scale.y;
    transform.translation.x += delta_x;
    transform.translation.y -= delta_y;
}
