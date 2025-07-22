use bevy::prelude::*;
use bevy::color::palettes::css::*;

use crate::player;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(Update, (update_score_display, try_again_button));
    }
}

/* ------------------- */
/*      components     */
/* ------------------- */
#[derive(Component)]
struct ScoreDisplay;

/* ------------------ */
/*      functions     */
/* ------------------ */

/* this just spawns the score text thingy on the bottom left, which will then be updated by update_score_display */
fn setup(mut commands: Commands) {
    commands.spawn((
        ScoreDisplay,

        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(5.),
            left: Val::Px(5.0),
            ..default()
        },
        Text::new("score: N/A"),
        TextColor::from(BLACK)
    ));
}

fn update_score_display(
    mut text: Single<&mut Text, With<ScoreDisplay>>,
    score: Res<player::PlayerScore>
) {
    **text = Text::new(format!("score: {}", score.0));
}

/* spawn a game over UI where the user can restart the game (called by player::handle_death) */
/* try again button logic is handled in try_again_button below */
pub fn spawn_gameover_ui(
    mut commands: Commands, 
    score: Res<player::PlayerScore>
) {
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(90.0),
            
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        children![
            (
                Node {
                    padding: UiRect {
                        bottom: Val::Percent(1.),
                        ..default()
                    },
                    ..default()
                },
                
                Text::new("woops, looks like you hit yourself!"),
                TextColor::from(BLACK)
            ),
            (
                Node {
                    padding: UiRect {
                        bottom: Val::Percent(1.),
                        ..default()
                    },
                    ..default()
                },
                TextFont {
                    font_size: 16.0,
                    ..default()
                },

                Text::new(format!("score: {}", score.0)),
                TextColor::from(BLACK)
            ),
            (
                Button,
                Text::new("try again"),
                TextColor::from(GREY)
            )
        ]
    ));
}

/* also changes the button's text color when hovered to make it look a bit nicer */
fn try_again_button(
    mut commands: Commands, 
    interaction_query: Query<(&Interaction, &mut TextColor), (Changed<Interaction>, With<Button>)>
) {
    for (interaction, mut text_color) in interaction_query {
        match *interaction {
            Interaction::Pressed => {
                commands.run_system_cached(crate::respawn_arena);
            }
            Interaction::Hovered => {
                *text_color = TextColor::from(BLACK);
            }
            Interaction::None => {
                *text_color = TextColor::from(GREY);
            }
        }
    }
}