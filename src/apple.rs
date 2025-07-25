use rand::Rng;

use bevy::prelude::*;
use bevy::color::palettes::css::*;

use crate::*;

pub struct ApplePlugin;
impl Plugin for ApplePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), spawn_apple);
        app.add_systems(OnExit(GameState::InGame), despawn_apple);
    }
}

/* ------------------- */
/*      components     */
/* ------------------- */
#[derive(Component)]
pub struct Apple();

/* ------------------ */
/*      functions     */
/* ------------------ */

/* generate a random position that is properly aligned/tiled with the playarea */
/* we convert a lot between ints and floats here because rand can only generate random ints */
/* but bevy expects floats, so this is a bit messy */
fn generate_random_position() -> Vec3 {
    let mut rng = rand::rng();

    /* convert tile_size to int as it makes the math easier */
    let tile_size: i32 = TILE_SIZE as i32;

    let mut x: i32 = rng.random_range(-PLAYAREA_X as i32..PLAYAREA_X as i32);
    let mut y: i32 = rng.random_range(-PLAYAREA_Y as i32..PLAYAREA_Y as i32);

    /* round up the random numbers to something divisible by tile_size (for proper tiling) */
    x = tile_size*((x + (tile_size-1))/tile_size);
    y = tile_size*((y + (tile_size-1))/tile_size);

    Vec3::new(x as f32, y as f32, 0.0)
}

/* ---------------- */
/*      systems     */
/* ---------------- */

/* spawns an apple at a random location */
pub fn spawn_apple(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,

    player_tail: Query<&Transform, (With<player::PlayerSegment>, Without<player::Player>)>,
) {
    let mut position = generate_random_position();

    /* prevent the apple from spawning on the tail */
    for segment_transform in player_tail.iter() {
        while segment_transform.translation == position {
            println!("WE ARE LOOPING!!!");
            position = generate_random_position();
        }
    }

    commands.spawn((
        Apple(),

        Mesh2d(meshes.add(Rectangle::from_length(TILE_SIZE))),
        MeshMaterial2d(materials.add(Color::from(INDIAN_RED))),
        Transform::from_translation(position)
    ));
}

fn despawn_apple(
    mut commands: Commands,
    apple: Single<Entity, With<Apple>>
) {
    commands.entity(*apple).despawn();
}