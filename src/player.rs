use bevy::prelude::*;
use bevy::color::palettes::css::*;

use crate::apple;
use crate::GameState;
use crate::{PLAYAREA_X, PLAYAREA_Y, TILE_SIZE};

/* in milliseconds, the higher the number the slower the player moves */
const MOVEMENT_SPEED: f32 = 0.15;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), spawn_player);
        app.add_systems(OnExit(GameState::InGame), despawn_player);

        app.add_systems(Update, (
            change_direction, 
            move_player, 
            detect_apple_collision, 
            detect_segment_collision,
        ).run_if(in_state(GameState::InGame)));

        app.insert_resource(PlayerScore(0));
    }
}

/* -------------- */
/*      enums     */
/* -------------- */
#[derive(Debug)]
enum PlayerDirection {
    Up,
    Down,
    Left,
    Right
}

/* ------------------- */
/*      components     */
/* ------------------- */

/* this component represents the "head" of the snake */
#[derive(Component)]
pub struct Player(PlayerDirection);

/* this component represents a piece of the snake's "tail" */
#[derive(Component)]
pub struct PlayerSegment();

/* ------------------ */
/*      resources     */
/* ------------------ */
#[derive(Resource)]
pub struct PlayerScore(pub u32);

/* ---------------- */
/*      systems     */
/* ---------------- */
fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        Player(PlayerDirection::Right),

        Mesh2d(meshes.add(Rectangle::from_length(TILE_SIZE))),
        MeshMaterial2d(materials.add(Color::from(LIGHT_GREEN))),
        Transform::default()
    ));

    /* we spawn one segment as well so that the player isnt just a single lonely cube */
    commands.run_system_cached(spawn_segment);
}

fn despawn_player(
    mut commands: Commands,
    query: Query<Entity, Or<(With<Player>, With<PlayerSegment>)>>,
) {
    for entity in query.iter() {
        commands.entity(entity).try_despawn();
    }
}

fn change_direction(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut player: Single<&mut Player>
) {
    if keyboard.pressed(KeyCode::KeyW) || keyboard.pressed(KeyCode::ArrowUp) {
        if let PlayerDirection::Down = player.0 {
            return;
        }

        player.0 = PlayerDirection::Up;
    }
    if keyboard.pressed(KeyCode::KeyS) || keyboard.pressed(KeyCode::ArrowDown) {
        if let PlayerDirection::Up = player.0 {
            return;
        }

        player.0 = PlayerDirection::Down;
    }
    if keyboard.pressed(KeyCode::KeyA) || keyboard.pressed(KeyCode::ArrowLeft) {
        if let PlayerDirection::Right = player.0 {
            return;
        }

        player.0 = PlayerDirection::Left;
    }
    if keyboard.pressed(KeyCode::KeyD) || keyboard.pressed(KeyCode::ArrowRight) {
        if let PlayerDirection::Left = player.0 {
            return;
        }

        player.0 = PlayerDirection::Right;
    }
}

fn move_player(
    player: Single<&Player>,
    mut player_transform: Single<&mut Transform, With<Player>>,
    mut player_tail: Query<&mut Transform, (With<PlayerSegment>, Without<Player>)>,

    mut movement_timer: Local<Option<Timer>>,
    time: Res<Time>,
) {
    /* initialize movement timer if it is empty */
    movement_timer.get_or_insert(Timer::from_seconds(MOVEMENT_SPEED, TimerMode::Repeating));

    /* slows down the player so that the player gets moved only once every MOVEMENT_SPEED seconds */
    /* movement_timer used to be a resource but then i found out about Locals and thought this would be better since that resource only ever got used in this system */
    /* using unwrap() here is fine as movement_timer is always valid as it was just initialized above */
    if !(movement_timer.as_mut().unwrap().tick(time.delta()).just_finished()) {
        return;
    }
    
    let mut prev_transform = player_transform.clone();

    /* move head */
    match player.0 {
        PlayerDirection::Up => player_transform.translation.y += TILE_SIZE,
        PlayerDirection::Down => player_transform.translation.y -= TILE_SIZE,
        PlayerDirection::Left => player_transform.translation.x -= TILE_SIZE,
        PlayerDirection::Right => player_transform.translation.x += TILE_SIZE
    }

    /* move segments */
    for mut segment_transform in player_tail.iter_mut() {
        let prev = segment_transform.clone();

        segment_transform.translation.x = prev_transform.translation.x;
        segment_transform.translation.y = prev_transform.translation.y;

        prev_transform = prev;
    }

    /* wrap around the map if we go out of bounds */
    if player_transform.translation.x > (PLAYAREA_X + TILE_SIZE) {
        player_transform.translation.x = -PLAYAREA_X;
    }
    if player_transform.translation.x < -(PLAYAREA_X + TILE_SIZE) {
        player_transform.translation.x = PLAYAREA_X;
    }

    if player_transform.translation.y > (PLAYAREA_Y + TILE_SIZE) {
        player_transform.translation.y = -PLAYAREA_Y;
    }
    if player_transform.translation.y < -(PLAYAREA_Y + TILE_SIZE) {
        player_transform.translation.y = PLAYAREA_Y;
    }
}

fn detect_apple_collision(
    mut commands: Commands,

    apple_query: Single<(&Transform, Entity), With<apple::Apple>>,
    player_transform: Single<&Transform, With<Player>>,

    mut score: ResMut<PlayerScore>
) {
    let (apple_transform, apple_entity) = apple_query.into_inner();

    /* we have collided with an apple */
    if player_transform.translation == apple_transform.translation {
        score.0 += 1;
        commands.entity(apple_entity).despawn();

        commands.run_system_cached(apple::spawn_apple);
        commands.run_system_cached(spawn_segment);
        println!("apples eaten: {}", score.0);
    }
}

/* this is where we extend the snake's tail */
fn spawn_segment(
    mut commands: Commands,

    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        PlayerSegment(),

        Mesh2d(meshes.add(Rectangle::from_length(TILE_SIZE))),
        MeshMaterial2d(materials.add(Color::from(LIGHT_GREEN))),
        /* set the transform somewhere where it cant be seen cuz player movement will deal with it anyway */
        Transform::from_xyz(10_000., 10_000., 0.)
    ));
}

/* detect if the player has hit their own tail, and set the gamestate to GameOver if we have */
fn detect_segment_collision(
    mut next_state: ResMut<NextState<GameState>>,

    player_transform: Single<&Transform, With<Player>>,
    player_tail: Query<&Transform, (With<PlayerSegment>, Without<Player>)>,
) {
    for segment_transform in player_tail.iter() {
        if player_transform.translation == segment_transform.translation {
            next_state.set(GameState::GameOver);
        }
    }
}