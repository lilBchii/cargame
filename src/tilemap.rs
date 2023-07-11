use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::car::{Car, spawn_car};
use crate::common::{LEVEL_COUNT, LVL0_P1_START, LVL1_P1_START, LVL2_P1_START, LVL3_P1_START, AppState};

/*
#[derive(Component, Clone, PartialEq, Eq, Debug, Default)]
pub enum LevelItem {
    #[default]
    None,
    Road,
    Sky,
    StartingLine,
}*/

pub fn setup_tilemap(mut commands: Commands, 
                     asset_server: Res<AssetServer>) 
{
    let bg: bevy::prelude::Handle<Image> = asset_server.load("spacebg.png");

    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("mapatlas.ldtk"),
        transform: Transform::from_xyz(0., 0., 10.),
        ..Default::default()
    });
    spawn_car(&mut commands, &asset_server, LVL0_P1_START.0, LVL0_P1_START.1);
}

pub fn switch_level(mut commands: Commands,
                    kb: Res<Input<KeyCode>>,
                    mut level_selection: ResMut<LevelSelection>,
                    q_car: Query<Entity, With<Car>>,
                    asset_server: Res<AssetServer>,
                    mut app_state: ResMut<NextState<AppState>>) 
{
    if kb.just_pressed(KeyCode::Space) {
        if let LevelSelection::Index(index) = *level_selection {

            // Despawn car
            for car in &q_car {
                commands.entity(car).despawn_recursive();
            }

            // Switch level
            if index as i32 == LEVEL_COUNT - 1 {
                *level_selection = LevelSelection::Index(0);
                app_state.set(AppState::StartMenu);
            } else {
                let n_index = index + 1;
                *level_selection = LevelSelection::Index(n_index);
                match n_index {
                    1 => spawn_car(&mut commands, &asset_server, LVL1_P1_START.0, LVL1_P1_START.1),
                    2 => spawn_car(&mut commands, &asset_server, LVL2_P1_START.0, LVL2_P1_START.1),
                    3 => spawn_car(&mut commands, &asset_server, LVL3_P1_START.0, LVL3_P1_START.1),
                    _ => {},
                }
            }
        }
    }
}

pub fn cleanup_ldtk_world(mut commands: Commands,
                          q_ldtk_world: Query<Entity, With<Handle<LdtkAsset>>>)
{
    for entity in &q_ldtk_world {
        commands.entity(entity).despawn_recursive();
    }
}

//pub fn reset_level_selection
