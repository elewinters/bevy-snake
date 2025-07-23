use bevy::prelude::*;
use bevy::color::palettes::css::*;

use crate::player;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(crate::SpawnSchedule, spawn_score_text);
        app.add_systems(Startup, spawn_start_menu);
        app.add_systems(Update, (
            update_score_display, 
            restart_button,
            quit_button,
            button_highlighting
        ));
    }
}

/* --------------- */
/*      macros     */
/* --------------- */
macro_rules! add_padding {
    ($padding:expr) => {
        Node {
            padding: UiRect {
                bottom: Val::Percent($padding),
                ..default()
            },
            ..default()
        }
    };
}

macro_rules! font_size {
    ($size:expr) => {
        TextFont {
            font_size: $size,
            ..default()
        }
    }
}

/* ------------------- */
/*      components     */
/* ------------------- */
#[derive(Component)]
struct ScoreDisplay;

#[derive(Component)]
struct RestartButton;

#[derive(Component)]
struct QuitButton;

/* ------------------ */
/*      functions     */
/* ------------------ */

/* this just spawns the score text thingy on the bottom left, which will then be updated by update_score_display */
fn spawn_score_text(mut commands: Commands) {
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

/* called only once at Startup */
/* button logic for these is handled in restart_button and quit_button */
fn spawn_start_menu(mut commands: Commands) {
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
                add_padding!(1.),

                Text::new("bevy_snake"),
                TextColor::from(BLACK)
            ),
            (
                add_padding!(1.),
                font_size!(16.),

                Text::new("a snake clone made in bevy 0.16"),
                TextColor::from(BLACK)
            ),
            (
                add_padding!(1.),
                font_size!(13.),

                Text::new("use WASD or arrow keys to move and collect apples to grow"),
                TextColor::from(BLACK)
            ),
            (
                add_padding!(3.),
                font_size!(13.),

                Text::new("try not to hit your tail!"),
                TextColor::from(BLACK)
            ),
            (
                add_padding!(2.),

                Button,
                RestartButton,

                Text::new("ok"),
                TextColor::from(GREY)
            ),
            (
                Button,
                QuitButton,

                Text::new("no thanks"),
                TextColor::from(GREY)
            )
        ]
    ));
}

/* spawn a game over UI where the user can restart the game (called by player::handle_death) */
/* try again button logic is handled in restart_button below */
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
                add_padding!(1.),
                
                Text::new("woops, looks like you hit yourself!"),
                TextColor::from(BLACK)
            ),
            (
                add_padding!(1.),
                font_size!(16.),

                Text::new(format!("score: {}", score.0)),
                TextColor::from(BLACK)
            ),
            (
                Button,
                RestartButton,

                Text::new("try again"),
                TextColor::from(GREY)
            )
        ]
    ));
}

/* this handles the "ok" button and the "try again" button, as both of them do the same thing of spawning all entities that we need */
fn restart_button(
    mut commands: Commands, 
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<RestartButton>)>
) {
    for interaction in interaction_query {
        if *interaction == Interaction::Pressed {
            commands.run_system_cached(crate::respawn_entities);
        }
    }
}

/* handles clicking the "no thanks" button */
fn quit_button(
    mut commands: Commands, 
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<QuitButton>)>
) {
    for interaction in interaction_query {
        if *interaction == Interaction::Pressed {
            commands.send_event(AppExit::Success);
        }
    }
}

/* change the text color of the button when hovered */
fn button_highlighting(
    interaction_query: Query<(&Interaction, &mut TextColor), (Changed<Interaction>, With<Button>)>
) {
    for (interaction, mut text_color) in interaction_query {
        match *interaction {
            Interaction::Hovered => {
                *text_color = TextColor::from(BLACK);
            }
            Interaction::None => {
                *text_color = TextColor::from(GREY);
            }
            Interaction::Pressed => ()
        }
    }
}