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
pub const PLAYAREA_X: f32 = 300.;
pub const PLAYAREA_Y: f32 = 300.;

/* play area has to be divisible by the tile_size for tiling to work */
pub const TILE_SIZE: f32 = 30.;

/* ------------------ */
/*      schedules     */
/* ------------------ */
#[derive(ScheduleLabel, Hash, Debug, PartialEq, Eq, Clone)]
pub struct SpawnSchedule;

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
    
    /* spawn everything that needs to be spawned */
    commands.run_schedule(SpawnSchedule);
}

pub fn despawn_all_entities(
    mut commands: Commands, 
    query: Query<Entity, Or<(With<Mesh2d>, With<Node>)>>
) {
    /* despawn all visible entities */
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn respawn_entities(
    mut commands: Commands,
    mut score: ResMut<player::PlayerScore>,
) {
    commands.run_system_cached(despawn_all_entities);
    score.0 = 0;

    commands.run_schedule(SpawnSchedule);
}