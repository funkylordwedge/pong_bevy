use bevy::prelude::*;
use crate::{GameTextures, WinSize, PADDLE_WIDTH_HALF, SPRITE_SCALE, TIME_STEP, PADDLE_SPEED, PADDLE1_SIZE, PADDLE2_SIZE, SCREEN_SIZE, PADDLE_HEIGHT_HALF, SCREEN_SCALE,};
use crate::components::*;

pub struct PaddlePlugin;

impl Plugin for PaddlePlugin {
    fn build(&self, app: &mut App)  {
        app
        .add_startup_system_to_stage(StartupStage::PostStartup, paddle_spawn_system)
        .add_system(paddle1_movement_system)
        .add_system(paddle2_movement_system)
        .add_system(paddle1_keyboard_event_system)
        .add_system(paddle2_keyboard_event_system)
        .add_system(paddle1_keyboard_event_system_cheat);
    }
}

fn clamp(value: &mut f32, low: f32, high: f32) {
    if *value < low {
        *value = low;
    } else if *value > high {
        *value = high;
    }
}

fn paddle_spawn_system(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    win_size: Res<WinSize>,
)   {
    let left = -win_size.w / 2.;
    let right = win_size.w / 2.;
    let top = win_size.h;
    let bottom = -win_size.h;
    commands.spawn
        (SpriteBundle {
            texture: game_textures.paddle1.clone(),
            transform: Transform {
            translation: Vec3::new(left + PADDLE_WIDTH_HALF +5., 0., 10.),
            scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(SpriteSize::from(PADDLE1_SIZE))
    .insert(Paddle1)
    .insert(Velocity {x:0.0, y:0.0});
    
    commands.spawn(
        SpriteBundle {
            texture: game_textures.paddle2.clone(),
            transform: Transform {
                translation: Vec3::new( right + PADDLE_WIDTH_HALF -117., 0., 10.),
                scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(SpriteSize::from(PADDLE2_SIZE))
        .insert(Paddle2)
        .insert(Velocity {x: 0.0, y: 0.0});
}

fn paddle1_keyboard_event_system(
    kb: Res<Input<KeyCode>>,
    mut query: Query<&mut Velocity, With<Paddle1>>,
)   {
    if let Ok(mut velocity) = query.get_single_mut() {
        velocity.y = if kb.pressed(KeyCode::W)  {
            1.
        } else if kb.pressed(KeyCode::S)  {
            -1.
        } else {
            0.
        }
    }
}

fn paddle1_keyboard_event_system_cheat(
    kb: Res<Input<KeyCode>>,
    mut query: Query<&mut Velocity, With<Paddle1>>,
)   {
    if let Ok(mut velocity) = query.get_single_mut() {
        velocity.x = if kb.pressed(KeyCode::D)  {
            1.
        } else if kb.pressed(KeyCode::A)  {
            -1.
        } else {
            0.
        }
    }
}

fn paddle2_keyboard_event_system(
    kb: Res<Input<KeyCode>>,
    mut query: Query<&mut Velocity, With<Paddle2>>,
)   {
    if let Ok(mut velocity) = query.get_single_mut() {
        velocity.y = if kb.pressed(KeyCode::Up)  {
            1.
        } else if kb.pressed(KeyCode::Down)  {
            -1.
        } else {
            0.
        }
    }
}

fn paddle1_movement_system(mut query: Query<(&Velocity, &mut Transform), With<Paddle1>>) {
    for (velocity, mut transform) in query.iter_mut() {
        let translation = &mut transform.translation;
        translation.x += velocity.x * TIME_STEP * PADDLE_SPEED;
        translation.y += velocity.y * TIME_STEP * PADDLE_SPEED;        
    }
}

fn paddle2_movement_system(mut query: Query<(&Velocity, &mut Transform), With<Paddle2>>) {
    for (velocity, mut transform) in query.iter_mut() {
        let translation = &mut transform.translation;
        translation.x += velocity.x * TIME_STEP * PADDLE_SPEED;
        translation.y += velocity.y * TIME_STEP * PADDLE_SPEED;
    }
}