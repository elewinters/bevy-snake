use bevy::prelude::*;
use bevy::color::palettes::css::*;
use bevy::ecs::schedule::ScheduleLabel;

mod player;
mod apple;
mod ui;

/* ------------------------- */
/*      global constants     */
/* ------------------------- */

/* window size is calculated based on the play area */
const PLAYAREA_X: f32 = 300.;
const PLAYAREA_Y: f32 = 300.;

/* play area has to be divisible by the tile_size for tiling to work */
const TILE_SIZE: f32 = 30.;

/* ------------------ */
/*      schedules     */
/* ------------------ */
#[derive(ScheduleLabel, Hash, Debug, PartialEq, Eq, Clone)]
struct SpawnSchedule;

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

        .insert_resource(ClearColor(Color::from(WHITE))) /* white background */
        .run();
}

fn setup(mut commands: Commands, mut window: Single<&mut Window>) {
    /* set up the window correctly */
    window.resolution.set(PLAYAREA_X * 2. + TILE_SIZE, PLAYAREA_Y * 2. + TILE_SIZE);
    window.title = String::from("snake");
    window.resizable = false;
    window.enabled_buttons.maximize = false;

    /* spawn camera with a white background (specified in main) */
    commands.spawn(Camera2d);
}

/* despawn all entities that have a Transform component (which are all entities that you can see in world space) */
/* with the exception of the Camera */
fn despawn_all_entities(
    mut commands: Commands, 
    query: Query<Entity, (With<Transform>, Without<Camera>)>
) {
    for entity in query.iter() {
        commands.entity(entity).try_despawn();
    }
}

fn respawn_entities(
    mut commands: Commands,
    mut score: ResMut<player::PlayerScore>,
) {
    commands.run_system_cached(despawn_all_entities);
    score.0 = 0;

    commands.run_schedule(SpawnSchedule);
}