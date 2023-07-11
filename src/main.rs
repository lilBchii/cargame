use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use tilemap::*;
use car::*;
use camera::*;
use ui::*;
use collision::*;

use crate::common::{AppState, MultiplayerMode};

mod tilemap;
mod car;
mod camera;
mod common;
mod ui;
mod collision;

fn main() 
{
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(LdtkPlugin)
        .add_state::<AppState>()
        .insert_resource(ClearColor(Color::rgb(0.,0.,0.)))
        .insert_resource(LevelSelection::Index(0))
        .insert_resource(LdtkSettings {
            level_spawn_behavior: LevelSpawnBehavior::UseZeroTranslation,
            ..Default::default()
        })
        .insert_resource(MultiplayerMode::SinglePlayer)
        .add_startup_system(setup_camera)
        .add_system(setup_start_menu.in_schedule(OnEnter(AppState::StartMenu)))
        .add_system(despawn_car.in_schedule(OnEnter(AppState::StartMenu)))
        .add_system(cleanup_ldtk_world.in_schedule(OnEnter(AppState::StartMenu)))
        .add_system(start_game.in_set(OnUpdate(AppState::StartMenu)))
        .add_system(switch_multiplayer_mode.in_set(OnUpdate(AppState::StartMenu)))
        .add_system(despawn_screen::<OnStartMenuScreen>.in_schedule(OnExit(AppState::StartMenu)))
        .add_system(setup_tilemap.in_schedule(OnEnter(AppState::Playing)))
        .add_system(car_movement.in_set(OnUpdate(AppState::Playing)))
        .add_system(camera_follow.in_set(OnUpdate(AppState::Playing)))
        .add_system(switch_level.in_set(OnUpdate(AppState::Playing)))
        .run();
}
