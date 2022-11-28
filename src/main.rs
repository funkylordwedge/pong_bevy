#![allow(unused)]

use bevy::prelude::*;
use crate::paddle::PaddlePlugin;
use crate::ball::BallPlugin;
use crate::components::*;
use rand::{self, thread_rng, Rng};

pub mod ball;
pub mod paddle;
pub mod components;
pub mod wall;

pub struct WinSize {
    pub w: f32,
    pub h: f32,
}

struct GameTextures {
    paddle1: Handle<Image>,
    paddle2: Handle<Image>,
    ball: Handle<Image>,
}
const TIME_STEP: f32 = 1.00 / 60.0;

const PADDLE1_SPRITE:  &str = "paddle2.png";
const PADDLE2_SPRITE: &str = "paddle2.png";
const PADDLE_WIDTH: f32 = 112.;
const PADDLE_HEIGHT: f32 = 234.;
const PADDLE_WIDTH_HALF: f32 = PADDLE_WIDTH / 2.;
const PADDLE_HEIGHT_HALF: f32 = PADDLE_HEIGHT / 2.;
const PADDLE1_SIZE: (f32, f32) = (PADDLE_WIDTH, PADDLE_HEIGHT);
const PADDLE2_SIZE: (f32, f32) = (PADDLE_WIDTH, PADDLE_HEIGHT);
const PADDLE_SPEED: f32 = 500.0;
const SPRITE_SCALE: f32 = 0.5;
const LEFT_WALL: f32 = -400.0;
const RIGHT_WALL: f32 = 400.0;
const BOTTOM_WALL: f32 = -300.0;
const TOP_WALL: f32 = 300.0;
const WALL_THICKNESS: f32 = 10.0;
const WALL_COLOR: Color = Color::rgb(255.0, 255.0, 255.0);
const GAP_BETWEEN_LEFT_WALL_AND_PADDLE: f32 = 5.0;
const GAP_BETWEEN_RIGHT_WALL_AND_PADDLE: f32 = -5.0;

const BALL_SPRITE: &str = "Ball1.png";
const BALL_WIDTH: f32 = 32.0;
const BALL_WIDTH_HALF: f32 = BALL_WIDTH / 2.;
const BALL_HEIGHT: f32 = 32.0;
const BALL_HEIGHT_HALF: f32 = BALL_HEIGHT / 2.;
const BALL_SPEED: f32 = 500.0;
const BALL_SIZE: (f32, f32) = (BALL_WIDTH, BALL_HEIGHT);


fn main() {
    App::new()
    .insert_resource(ClearColor(Color::rgb(0.00, 0.00, 0.00)))
    .insert_resource(WindowDescriptor {
        width: 800.0,
        height: 600.0,
        title: "UnderPong!".to_string(),
        ..Default::default()
    })
    .add_plugins(DefaultPlugins)
    .add_plugin(BallPlugin)
    .add_plugin(PaddlePlugin)
    .add_startup_system(setup_system)
    .run();
}

fn setup_system(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
    mut windows: ResMut<Windows>,
) {
    // camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    
    // capture window size
    let window = windows.get_primary_mut().unwrap();
    let (win_w, win_h) = (window.width(), window.height());

    // position window (for tutorial)
    // window.set_position(IVec2::new(2780, 4900));
    // commenting this out - seems to be part of tutorial but 
    // not sure if I might need to use later. 

    // add WinSize resource
    let win_size = WinSize {w: win_w, h: win_h};
    commands.insert_resource(win_size);

    // add GameTextures resource
    let game_textures = GameTextures {
        paddle1: asset_server.load(PADDLE1_SPRITE),
        paddle2: asset_server.load(PADDLE2_SPRITE),
        ball: asset_server.load(BALL_SPRITE),
    };
    commands.insert_resource(game_textures);
}

