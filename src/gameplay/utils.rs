use bevy::{
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
    sprite::MaterialMesh2dBundle,
};
use bevy_rapier2d::prelude::*;
use crate::GAME_WIDTH;

const GRAVITY_SCALE_DEFAULT: f32 = 0.4;
const VELOCITY_DEFAULT: f32 = 0.0;

pub fn create_sprite_bundle(
    texture: Handle<Image>,
    (x_size, y_size): (f32, f32),
    (x_translation, y_translation, z_translation): (f32, f32, f32),
) -> SpriteBundle {
    SpriteBundle {
        texture,
        sprite: Sprite {
            custom_size: Some(Vec2::new(x_size, y_size)),
            ..default()
        },
        transform: Transform::from_xyz(x_translation, y_translation, z_translation),
        ..default()
    }
}

pub fn spawn_dynamic_object(
    commands: &mut Commands,
    sprite: SpriteBundle,
    x_velocity: Option<f32>,
    gravity_scale: Option<f32>,
) -> Entity {
    commands
        .spawn_bundle(sprite)
        .insert(RigidBody::Dynamic)
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(Sleeping::disabled())
        .insert(GravityScale(gravity_scale.unwrap_or(GRAVITY_SCALE_DEFAULT)))
        .insert(Ccd::enabled())
        .insert(Velocity::linear(Vec2::new(
            x_velocity.unwrap_or(VELOCITY_DEFAULT),
            VELOCITY_DEFAULT,
        )))
        .id()
}

pub fn spawn_solid_collider(
    commands: &mut Commands,
    entity: Entity,
    collider: Collider,
    friction: Option<Friction>,
) -> Entity {
    commands
        .entity(entity)
        .insert(collider)
        .insert(friction.unwrap_or_default())
        .insert(ActiveEvents::COLLISION_EVENTS)
        .id()
}

pub struct ScoreboardPlugin;

impl Plugin for ScoreboardPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Scoreboard::new());
    }
}

struct Scoreboard {
    score1: usize,
    score2: usize,
}

impl Scoreboard {
    fn new() -> Scoreboard {
        Scoreboard {
            score1: 0,
            score2: 0,
        }
    }
}

// #[derive(Resource)]
struct HealthBar {
    //todo pasek zdrowia 1
    //todo pasek zdrowia 2
}

pub struct Bounds;

const BONDS_FLOOR: f32 = -200.0;
const BONDS_LEFT_WALL: f32 = - GAME_WIDTH / 2.0;
const BONDS_RIGHT_WALL: f32 = GAME_WIDTH / 2.0;

impl Bounds {
    pub fn check_bounds_y(transform: &mut Transform) -> bool
    {
        if transform.translation.y < BONDS_FLOOR {
            transform.translation.y = BONDS_FLOOR;
            return true;
        }
        return false;
    }

    pub fn check_bounds_x(transform: &mut Transform) -> bool {
        if transform.translation.x < BONDS_LEFT_WALL {
            transform.translation.x = BONDS_LEFT_WALL;
            return true;
        }
        if transform.translation.x > BONDS_RIGHT_WALL {
            transform.translation.x = BONDS_RIGHT_WALL;
            return true;
        }
        return false;
    }
}


