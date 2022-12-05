use bevy::prelude::*;
use bevy::math::Vec2;
use bevy::sprite::collide_aabb::{collide, Collision};
use bevy::sprite::SpriteBundle;
use crate::{GameTextures, WinSize, WALL_THICKNESS, WALL_COLOR};
use crate::Transform;

use crate::paddle;
use crate::ball;

use paddle::*;
use ball::*;

#[derive(Default)]
pub struct CollisionEvent;

// region: --- Common Components
#[derive(Component)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

// region: --- Paddle Components
#[derive(Component)]
pub struct Paddle1;

#[derive(Component)]
pub struct Paddle2;

#[derive(Component)]
pub struct Ball;

#[derive(Component)]
pub struct SpriteSize(pub Vec2);

#[derive(Component)]
pub struct Collider; 

pub struct GameData {
    pub score: i32,
}


impl From<(f32, f32)> for SpriteSize {
    fn from(val: (f32, f32)) -> Self {
        SpriteSize(Vec2::new(val.0, val.1))
    }
}
