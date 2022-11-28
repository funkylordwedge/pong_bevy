use crate::{GameTextures, WinSize, TIME_STEP, BALL_SPEED, BALL_HEIGHT, BALL_HEIGHT_HALF, BALL_WIDTH, BALL_WIDTH_HALF, BALL_SIZE};
use bevy::prelude::*;
use bevy::sprite::collide_aabb::*;
use crate::components::*;

pub struct BallPlugin;


impl Plugin for BallPlugin {
    fn build(&self, app: &mut App)  {
        app
        .add_startup_system_to_stage(StartupStage::PostStartup, ball_spawn_system)
        .add_system(ball_movement_system);
    }
}


fn ball_spawn_system(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    win_size: Res<WinSize>,
) { 
    let left = -win_size.w / 2.;
    let right = win_size.w / 2.;
    commands.spawn_bundle(SpriteBundle {
        texture: game_textures.ball.clone(),
        ..Default::default()
    })
    .insert(Ball)
    .insert(SpriteSize::from(BALL_SIZE))
    .insert(Velocity {x:0.0, y:0.0});;
}

fn ball_movement_system(mut query: Query<&mut Transform, With<Ball>>) {
    for mut transform in query.iter_mut() {
        let translation = &mut transform.translation;
        translation.x += BALL_SPEED * TIME_STEP / 4.;
        translation.y += BALL_SPEED * TIME_STEP / 4.;
    }
}

