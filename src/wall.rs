use bevy::prelude::*;
use bevy::sprite::collide_aabb::{collide, Collision};
use crate::components::Wall;
use crate::{GameTextures, WinSize, TOP_WALL, BOTTOM_WALL, LEFT_WALL, RIGHT_WALL, WALL_THICKNESS};

