use bevy::prelude::*;
use bevy::color::palettes::css::*;

use crate::*;

pub struct UIPlugin;
impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        /* spawning */
        app.add_systems(OnEnter(GameState::MainMenu), spawn_main_menu);
        app.add_systems(OnEnter(GameState::InGame), spawn_score_display);
        app.add_systems(OnEnter(GameState::GameOver), spawn_gameover_ui);

        /* only update score display if in game */
        app.add_systems(Update, update_score_display.run_if(in_state(GameState::InGame)));

        /* we update these regardless what state we're in as these systems should run in both MainMenu and GameOver states */
        app.add_systems(Update, (
            button_highlighting,
            ingame_button,
            quit_button,
        ));
    }
}

/* ------------------ */
/*      constants     */
/* ------------------ */
const NORMAL_BUTTON_COLOR: Srgba = GREY;
const HOVERED_BUTTON_COLOR: Srgba = BLACK;

/* --------------- */
/*      macros     */
/* --------------- */
macro_rules! add_padding {
    ($padding:expr) => {
        Node {
            padding: UiRect {
                bottom: percent($padding),
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
struct InGameButton;

#[derive(Component)]
struct QuitButton;

/* ---------------- */
/*      systems     */
/* ---------------- */

/* button logic for these is handled in ingame_button and quit_button */
fn spawn_main_menu(mut commands: Commands) {
    let mut menu = commands.spawn((
        DespawnOnExit(GameState::MainMenu),

        Node {
            width: percent(100.0),
            height: percent(90.0),
            
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        children![
            (
                add_padding!(1.0),

                Text::new("bevy_snake v1.2.0"),
                TextColor::from(BLACK)
            ),
            (
                add_padding!(1.0),
                font_size!(16.0),

                Text::new("a snake clone made in bevy 0.17"),
                TextColor::from(BLACK)
            ),
            (
                add_padding!(1.0),
                font_size!(13.0),

                Text::new("use WASD or arrow keys to move and collect apples to grow"),
                TextColor::from(BLACK)
            ),
            (
                add_padding!(3.0),
                font_size!(13.0),

                Text::new("try not to hit your tail!"),
                TextColor::from(BLACK)
            ),
            (
                add_padding!(2.0),

                Button,
                InGameButton,

                Text::new("ok"),
                TextColor::from(NORMAL_BUTTON_COLOR)
            ),
        ]
    ));

    /* do not spawn the "no thanks" button on wasm, as this button just crashes the tab on wasm */
    #[cfg(not(target_arch = "wasm32"))]
    menu.with_child((
        Button,
        QuitButton,

        Text::new("no thanks"),
        TextColor::from(NORMAL_BUTTON_COLOR)
    ));
}

/* this just spawns the score text thingy on the bottom left, which will then be updated by update_score_display */
fn spawn_score_display(mut commands: Commands) {
    commands.spawn((
        DespawnOnExit(GameState::InGame),
        ScoreDisplay,

        Node {
            position_type: PositionType::Absolute,
            bottom: px(5.0),
            left: px(5.0),
            ..default()
        },
        Text::new("score: N/A"),
        TextColor::from(BLACK)
    ));
}

/* try again button logic is handled in ingame_button below */
fn spawn_gameover_ui(
    mut commands: Commands, 
    score: Res<player::PlayerScore>
) {
    commands.spawn((
        DespawnOnExit(GameState::GameOver),

        Node {
            width: percent(100.0),
            height: percent(90.0),
            
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        children![
            (
                add_padding!(1.0),
                
                Text::new("woops, looks like you hit yourself!"),
                TextColor::from(BLACK)
            ),
            (
                add_padding!(1.0),
                font_size!(16.0),

                Text::new(format!("score: {}", score.0)),
                TextColor::from(BLACK)
            ),
            (
                Button,
                InGameButton,

                Text::new("try again"),
                TextColor::from(NORMAL_BUTTON_COLOR)
            )
        ]
    ));
}

fn update_score_display(
    mut text: Single<&mut Text, With<ScoreDisplay>>,
    score: Res<player::PlayerScore>
) {
    **text = Text::new(format!("score: {}", score.0));
}

/* change the text color of the button when hovered */
fn button_highlighting(
    interaction_query: Query<(&Interaction, &mut TextColor), (Changed<Interaction>, With<Button>)>
) {
    for (interaction, mut text_color) in interaction_query {
        match *interaction {
            Interaction::Hovered => {
                *text_color = TextColor::from(HOVERED_BUTTON_COLOR);
            }
            Interaction::None => {
                *text_color = TextColor::from(NORMAL_BUTTON_COLOR);
            }
            Interaction::Pressed => ()
        }
    }
}

/* this handles the "ok" button and the "try again" button, as both of them do the same thing of setting the state to InGame */
fn ingame_button(
    mut next_state: ResMut<NextState<GameState>>,
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<InGameButton>)>,
) {
    for interaction in interaction_query {
        if *interaction == Interaction::Pressed {
            next_state.set(GameState::InGame);
        }
    }
}

/* handles clicking the "no thanks" button */
/* this button doesn't exist on wasm but thats fine as this system just wont be ran on wasm */
fn quit_button(
    mut commands: Commands, 
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<QuitButton>)>
) {
    for interaction in interaction_query {
        if *interaction == Interaction::Pressed {
            commands.write_message(AppExit::Success);
        }
    }
}