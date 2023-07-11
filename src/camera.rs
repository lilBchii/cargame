use bevy::prelude::*;

use crate::{car::Car,
            common::{CAMERA_SCALE, LEVEL_HEIGHT, LEVEL_WIDTH}};

pub fn setup_camera(mut commands: Commands) 
{
    let mut camera2d_bundle = Camera2dBundle::default();
    camera2d_bundle.projection.scale = CAMERA_SCALE;
//    camera2d_bundle.transform.translation.x = 1200.;
//    camera2d_bundle.transform.translation.y = 720.;
    commands.spawn(camera2d_bundle);
}

const CAMERA_W: f32 = 500. * 1.3 * CAMERA_SCALE;
const CAMERA_H: f32 = 260. * 1.3 * CAMERA_SCALE;

pub fn camera_follow(q_car: Query<&Transform, With<Car>>,
                 mut q_camera: Query<&mut Transform, (With<Camera>, Without<Car>)>) 
{
    let car_position = q_car.single().translation.truncate();
    let mut camera_transform = q_camera.single_mut();

    camera_transform.translation.x = car_position.x.clamp(CAMERA_W, LEVEL_WIDTH - CAMERA_W);
    camera_transform.translation.y = car_position.y.clamp(CAMERA_H, LEVEL_HEIGHT - CAMERA_H);
}

