/* disable console window on windows when in release mode */
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::prelude::*;
use bevy::color::palettes::css::*;

mod player;
mod apple;
mod ui;

/* ------------------ */
/*      constants     */
/* ------------------ */

/* window size is calculated based on the play area */
pub const PLAYAREA_X: f32 = 300.;
pub const PLAYAREA_Y: f32 = 300.;

/* play area has to be divisible by the tile_size for tiling to work */
pub const TILE_SIZE: f32 = 30.;

/* --------------- */
/*      states     */
/* --------------- */
#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
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
    window.resolution.set(PLAYAREA_X * 2. + TILE_SIZE, PLAYAREA_Y * 2. + TILE_SIZE);
    window.title = String::from("snake");
    window.resizable = false;
    window.enabled_buttons.maximize = false;

    /* spawn camera with a white background (specified in main) */
    commands.spawn(Camera2d);
}