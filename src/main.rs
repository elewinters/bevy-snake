/* disable console window on windows when in release mode */
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

/* dont warn about complex types (complex types are very common in bevy nd this is fine) */
#![allow(clippy::type_complexity)]

use bevy::prelude::*;
use bevy::color::palettes::css::*;

mod player;
mod apple;
mod ui;

/* ------------------ */
/*      constants     */
/* ------------------ */

/* window size is calculated based on the play area */
pub const PLAYAREA_X: f32 = 300.0;
pub const PLAYAREA_Y: f32 = 300.0;

/* play area has to be divisible by the tile_size for tiling to work */
pub const TILE_SIZE: f32 = 30.0;

/* --------------- */
/*      states     */
/* --------------- */
#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
#[states(scoped_entities)]
pub enum GameState {
    #[default]
    MainMenu,
    InGame,
    GameOver,
}

/* ------------------ */
/*      functions     */
/* ------------------ */
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((
            player::PlayerPlugin,
            apple::ApplePlugin,
            ui::UIPlugin
        ))
        .add_systems(Startup, setup)
        .init_state::<GameState>()

        .insert_resource(ClearColor(Color::from(WHITE))) /* white background */
        .run();
}

/* ---------------- */
/*      systems     */
/* ---------------- */
fn setup(mut commands: Commands, mut window: Single<&mut Window>) {
    /* set up the window correctly */
    window.resolution.set(PLAYAREA_X * 2.0 + TILE_SIZE, PLAYAREA_Y * 2.0 + TILE_SIZE);
    window.title = String::from("snake");
    window.resizable = false;
    window.enabled_buttons.maximize = false;

    /* spawn camera with a white background (specified in main) */
    commands.spawn(Camera2d);
}