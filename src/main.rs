use bevy::prelude::*;
use bevy::color::palettes::css::*;

mod player;
mod apple;
mod ui;

/* ------------------------- */
/*      global constants     */
/* ------------------------- */

/* window size is calculated based on the play area */
pub const PLAYAREA_X: f32 = 300.;
pub const PLAYAREA_Y: f32 = 300.;

/* play area has to be divisible by the tile_size for tiling to work */
pub const TILE_SIZE: f32 = 30.;

/* --------------- */
/*      events     */
/* --------------- */
#[derive(Event)]
pub struct CleanUpArenaEvent;

#[derive(Event)]
pub struct RespawnArenaEvent;

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
        .add_observer(clean_up_arena)
        .add_observer(respawn_arena)

        .insert_resource(ClearColor(Color::from(WHITE))) /* white background */
        .run();
}

/* this runs when the player resets the arena even though it probably shouldn't */
/* but that's probably fine */
fn setup(mut commands: Commands, mut window: Single<&mut Window>) {
    /* set up the window correctly */
    window.resolution.set(PLAYAREA_X * 2. + TILE_SIZE, PLAYAREA_Y * 2. + TILE_SIZE);
    window.title = String::from("snake");
    window.resizable = false;
    window.enabled_buttons.maximize = false;

    /* spawn camera with a white background (specified in main) */
    commands.spawn(Camera2d);
}

fn clean_up_arena(
    _trigger: Trigger<CleanUpArenaEvent>,
    mut commands: Commands, 
    query: Query<Entity, Or<(With<Mesh2d>, With<Node>)>>
) {
    /* despawn all visible entities */
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

fn respawn_arena(
    _trigger: Trigger<RespawnArenaEvent>,
    mut commands: Commands,

    mut score: ResMut<player::PlayerScore>,
    camera: Single<Entity, With<Camera2d>>
) {
    commands.trigger(CleanUpArenaEvent);
    commands.entity(*camera).despawn(); /* we despawn the camera as a new one will get created once main's startup runs, which will cause multiple cameras to exist */
    score.0 = 0;

    commands.run_schedule(Startup);
}