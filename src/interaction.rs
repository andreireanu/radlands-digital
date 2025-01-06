use bevy::prelude::*;

#[derive(Component)]
pub struct Zoomable;

pub fn on_drag(drag: Trigger<Pointer<Drag>>, mut transforms: Query<&mut Transform>) {
    if let Ok(mut transform) = transforms.get_mut(drag.entity()) {
        let sensitivity = 1.; // Adjust for desired movement speed
        let delta_x = drag.delta.x * sensitivity;
        let delta_y = drag.delta.y * sensitivity;
        transform.translation.x += delta_x;
        transform.translation.y -= delta_y;
    }
}

pub fn on_right_click_down(click: Trigger<Pointer<Down>>, mut transforms: Query<&mut Transform>) {
    if let Ok(mut transform) = transforms.get_mut(click.entity()) {
        println!("{:?}", click);
        if click.event().button == PointerButton::Secondary {
            println!("Right click down");
            println!("{:?}", transform);
            transform.scale = Vec3::splat(2.0);
        }
    }
}

pub fn on_right_click_up(click: Trigger<Pointer<Up>>, mut transforms: Query<&mut Transform>) {
    if let Ok(mut transform) = transforms.get_mut(click.entity()) {
        println!("{:?}", click);
        if click.event().button == PointerButton::Secondary {
            println!("Right click down");
            println!("{:?}", transform);
            transform.scale = Vec3::splat(1.0);
        }
    }
}
