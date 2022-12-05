use crate::ball::*;
use crate::paddle::*;
use crate::ATARI_RES_X_VAL;
use crate::ATARI_RES_Y_VAL;
use bevy::prelude::*;

const SCOREBOARD_FONT_SIZE: f32 = 40.0;
const TEXT_COLOR: Color = Color::rgb(255.0, 255.0, 255.0);
const SCORE_COLOR: Color = Color::rgb(255.0, 255.0, 255.0);
const SCOREBOARD_TEXT_PADDING_Y: Val = ATARI_RES_Y_VAL;
const SCOREBOARD_TEXT_PADDING_X: Val = ATARI_RES_X_VAL;

#[derive(Resource, Default)]
pub struct Scoreboard {
    pub player1_score: usize,
    pub player2_score: usize,
}

pub struct ScoreboardPlugin;

impl Plugin for ScoreboardPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Scoreboard>()
            .add_startup_system(spawn)
            .add_system(update_scoreboard);
    }
}

fn spawn(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut scoreboard: ResMut<Scoreboard>,
    asset_server: Res<AssetServer>,
) {
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(
                TextBundle::from_sections([TextSection::new(
                    format!("{}  {}", scoreboard.player1_score, scoreboard.player2_score),
                    TextStyle {
                        font: asset_server.load("bit5x3.ttf"),
                        font_size: SCOREBOARD_FONT_SIZE,
                        color: TEXT_COLOR,
                    },
                )])
                .with_style(Style {
                    position_type: PositionType::Relative,
                    position: UiRect {
                        top: SCOREBOARD_TEXT_PADDING_Y,
                        ..default()
                    },
                    align_self: AlignSelf::Center,
                    ..default()
                }),
            );
        });
}

fn update_scoreboard(scoreboard: Res<Scoreboard>, mut query: Query<&mut Text>) {
    let mut text = query.single_mut();
    if let Some(text_section) = text.sections.get_mut(0) {
        text_section.value = format!("{}  {}", scoreboard.player1_score, scoreboard.player2_score);
    }
}
