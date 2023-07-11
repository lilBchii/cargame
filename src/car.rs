use bevy::prelude::*;

use crate::common::{CAR_SPEED, CAR_ROTATION, CAR_ACCELERATION, CAR_BRAKE};
use crate::collision::*;


#[derive(Component)]
pub struct Car {
    max_speed: f32,
    rotation_speed: f32,
    translation_delta: f32,
    acceleration: f32,
    brake: f32,
    hitbox: Hitbox,
}

/*
#[derive(Component)]
pub struct Player {
    pub car: Car,
    num: f32,
}*/

pub fn spawn_car(commands: &mut Commands,
                 asset_server: &Res<AssetServer>,
                 posx: f32,
                 posy: f32) 
{

    commands.spawn(SpriteBundle{
        texture: asset_server.load("car.png"),
        transform: Transform::from_translation(Vec3::new(posx, posy, 100.)),
        ..default()
    }).insert(Car{
            max_speed: CAR_SPEED,
            rotation_speed: CAR_ROTATION,
            translation_delta: 0.,
            acceleration: CAR_ACCELERATION,
            brake: CAR_BRAKE,
            hitbox: Hitbox::from(32.,10.),
    });
}

pub fn car_movement(mut query: Query<(&mut Car, &mut Transform), Without<Camera>>,
                kb: Res<Input<KeyCode>>,
                time: Res<Time>) 
{
    for (mut car, mut trans) in query.iter_mut() {
        
//        let trans = car.transform;

        // Car rotation
        let mut rotation_factor: f32 = 0.0;

        if kb.pressed(KeyCode::Right) {
            rotation_factor -= 1.0;
        } 
        if kb.pressed(KeyCode::Left) {
            rotation_factor += 1.0;
        }

        // Apply rotation
        trans.rotate_z(rotation_factor * car.rotation_speed * time.delta_seconds());
        let movement_direction = trans.rotation * Vec3::Y;

        // Car translation
        if kb.pressed(KeyCode::Up) {
            car.translation_delta += car.acceleration;
        }
        if kb.pressed(KeyCode::Down) {
            car.translation_delta *= car.brake;
        }

        // Apply car translation
        car.translation_delta = car.translation_delta.clamp(0.,car.max_speed);
        trans.translation += car.translation_delta * movement_direction * 3. * time.delta_seconds();

        // Deceleration
        car.translation_delta *= 0.98;
    }
}

pub fn despawn_car(mut commands: Commands,
                   q_car: Query<Entity, With<Car>>)
{
    for car in &q_car {
        commands.entity(car).despawn_recursive();
    }
}
