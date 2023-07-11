use bevy::prelude::*;

pub const CAR_SPEED: f32 = 200.0;
pub const CAR_ROTATION: f32 = 1.9;
pub const CAR_ACCELERATION: f32 = 2.8;
pub const CAR_BRAKE: f32 = 0.95;

pub const CAMERA_SCALE: f32 = 0.7;

//pub const TILE_SIZE: i32 = 24;
pub const LEVEL_COUNT: i32 = 4;
pub const LEVEL_HEIGHT: f32 = 1440.;
pub const LEVEL_WIDTH: f32 = 2400.;
pub const LVL0_P1_START: (f32, f32) = (1670., 702.);
pub const LVL1_P1_START: (f32, f32) = (2004., 770.);
pub const LVL2_P1_START: (f32, f32) = (2004., 680.);
pub const LVL3_P1_START: (f32, f32) = (1935., 655.);

#[derive(Debug, Clone, Eq, PartialEq, Hash, States, Default)]
pub enum AppState {
    #[default]
    StartMenu,
    Playing,
    GameOver,
}

#[derive(Resource, Debug, PartialEq, Eq)]
pub enum MultiplayerMode {
    SinglePlayer,
    TwoPlayers,
}

