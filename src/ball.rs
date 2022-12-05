use crate::components::*;
use crate::paddle::*;
use crate::scoreboard::*;
use crate::SCREEN_SIZE;
use crate::{
    GameTextures, WinSize, BALL_HEIGHT, BALL_HEIGHT_HALF, BALL_SIZE, BALL_SPEED, BALL_WIDTH,
    BALL_WIDTH_HALF, PADDLE1_SIZE, PADDLE2_SIZE, PADDLE_HEIGHT_HALF, PADDLE_WIDTH_HALF, TIME_STEP,
};
use bevy::prelude::*;

pub struct BallPlugin;
pub struct BallRespawnEvent;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, ball_spawn_system)
            .add_event::<BallRespawnEvent>()
            .add_system(ball_movement_system)
            .add_system(collide_paddle)
            .add_system(collide_top_bottom)
            .add_system(collide_left)
            .add_system(collide_right)
            .add_system(respawn);
    }
}

fn ball_spawn_system(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    win_size: Res<WinSize>,
) {
    let left = -win_size.w / 2.;
    let right = win_size.w / 2.;
    commands
        .spawn(SpriteBundle {
            texture: game_textures.ball.clone(),
            ..Default::default()
        })
        .insert(Ball)
        .insert(SpriteSize::from(BALL_SIZE))
        .insert(Velocity { x: 1.0, y: 1.0 });
}

fn ball_movement_system(mut balls: Query<(&mut Transform, &Velocity), With<Ball>>) {
    for (mut transform, ball_velocity) in balls.iter_mut() {
        let translation = &mut transform.translation;
        translation.x += ball_velocity.x * TIME_STEP / 4. * 250.0;
        translation.y += ball_velocity.y * TIME_STEP / 4. * 250.0;
    }
}


pub fn is_offscreen_top_bottom(point: Vec3) -> bool {
    let half_height = SCREEN_SIZE.y / 2.0;

    point.y < -half_height || point.y > half_height
}

pub fn is_offscreen_left(point: Vec3) -> bool {
    let half_width = SCREEN_SIZE.x / 2.0;

    point.x < -half_width
}

pub fn is_offscreen_right(point: Vec3) -> bool {
    let half_width = SCREEN_SIZE.x / 2.0;

    point.x > half_width
}


fn collide_paddle(
    mut ball: Query<
        (&mut Transform, &mut Velocity, &SpriteSize),
        (With<Ball>, Without<Paddle1>, Without<Paddle2>),
    >,
    paddle1: Query<(&Transform, &SpriteSize), (With<Paddle1>, Without<Paddle2>, Without<Ball>)>,
    paddle2: Query<(&Transform, &SpriteSize), (With<Paddle2>, Without<Paddle1>, Without<Ball>)>,
) {
    if ball.is_empty() || paddle1.is_empty() || paddle2.is_empty() {
        return;
    }

    let (mut ball_transform, mut ball_velocity, ball_size) = ball.single_mut();
    let (paddle1_transform, paddle1_size) = paddle1.single();
    let (paddle2_transform, paddle2_size) = paddle2.single();
    let ball_rect = Rect::from_center_size(ball_transform.translation.truncate(), Vec2::new(BALL_SIZE.0, BALL_SIZE.1));
    let paddle1_rect =  Rect::from_center_size(paddle1_transform.translation.truncate(), Vec2::new(PADDLE1_SIZEl.0, PADDLE1_SIZE.0);
    let paddle2_rect = Rect::from_center_size(paddle2_transform.translation.truncate(), PADDLE2_SIZE);


    if ball_rect.intersect(paddle1_rect) || ball_rect.intersect(paddle2_rect)
    {
        ball_velocity.x = -ball_velocity.x;
    }
}

fn collide_top_bottom(
    win_size: Res<WinSize>,
    mut ball: Query<(&Transform, &mut Velocity), With<Ball>>,
) {
    if ball.is_empty() {
        return;
    }

    let (ball_transform, mut ball_velocity) = ball.single_mut();

    if is_offscreen_top_bottom(ball_transform.translation) {
        ball_velocity.y = -ball_velocity.y;
    } else {
    }
}

fn collide_left(
    mut commands: Commands,
    win_size: Res<WinSize>,
    mut scoreboard: ResMut<Scoreboard>,
    mut respawn_event: EventWriter<BallRespawnEvent>,
    mut ball: Query<(Entity, &Transform, &mut Velocity), With<Ball>>,
) {
    if ball.is_empty() {
        return;
    }

    let (ball_entity, ball_transform, mut ball_velocity) = ball.single_mut();

    if is_offscreen_left(ball_transform.translation) {
        scoreboard.player2_score += 1;
        commands.entity(ball_entity).despawn();
        respawn_event.send(BallRespawnEvent);
    } else {
    }
}

fn collide_right(
    mut commands: Commands,
    win_size: Res<WinSize>,
    mut scoreboard: ResMut<Scoreboard>,
    mut respawn_event: EventWriter<BallRespawnEvent>,
    mut ball: Query<(Entity, &Transform, &mut Velocity), With<Ball>>,
) {
    if ball.is_empty() {
        return;
    }

    let (ball_entity, ball_transform, mut ball_velocity) = ball.single_mut();

    if is_offscreen_right(ball_transform.translation) {
        scoreboard.player1_score += 1;
        commands.entity(ball_entity).despawn();
        respawn_event.send(BallRespawnEvent);
    } else {
    }
}

fn test_system(transforms: Query<&Transform, With<Ball>>) {
    if transforms.is_empty() {
        return;
    }

    let ball_transform = transforms.single();

    for transform in transforms.iter() {}
}

 fn respawn(
    commands: Commands,
    game_textures: Res<GameTextures>,
    win_size: Res<WinSize>,
    mut respawn_event: EventReader<BallRespawnEvent>,
) {
    if respawn_event.iter().next().is_none() {
        return;
    }

    ball_spawn_system(commands, game_textures, win_size);
}