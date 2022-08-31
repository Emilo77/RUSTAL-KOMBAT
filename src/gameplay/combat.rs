use bevy::prelude::*;
use crate::gameplay::{Bounds, Player};

pub struct CombatPlugin;

pub struct PowerupPlugin;

const GRAVITY_CONST: f32 = 0.2;

//Dash
const DASH_COOLDOWN: f32 = 1.0;
const DASH_SPEED_BONUS: f32 = 10.0;
const DASH_DURATION: f32 = 0.25;
const DASH_DAMAGE: f32 = 10.0;
//Jump
const JUMP_POWER: f32 = 10.0;
//Strike
const STRIKE_COOLDOWN: f32 = 0.3;
const STRIKE_DAMAGE: f32 = 3.0;

#[derive(Component)]
pub struct Dash {
    cooldown: f32,
    speed_bonus: f32,
    duration: f32,
    damage: f32,
    pub is_active: bool,
}

#[derive(Component)]
pub struct Jump {
    power: f32,
    current_power: f32,
    pub is_active: bool,
}

#[derive(Component)]
pub struct Strike {
    cooldown: f32,
    damage: f32,
    pub is_active: bool,
}

#[derive(Component)]
pub struct Abilities {
    pub jump: Jump,
    pub dash: Dash,
    pub strike: Strike,
}

impl Abilities {
    pub fn default() -> Self {
        Abilities {
            jump: Jump {
                power: JUMP_POWER,
                current_power: JUMP_POWER,
                is_active: false,
            },
            dash: Dash {
                cooldown: DASH_COOLDOWN,
                speed_bonus: DASH_SPEED_BONUS,
                duration: DASH_DURATION,
                damage: DASH_DAMAGE,
                is_active: false,
            },
            strike: Strike {
                cooldown: STRIKE_COOLDOWN,
                damage: STRIKE_DAMAGE,
                is_active: false,
            },
        }
    }

    pub fn jump_possible(&self) -> bool {
        !self.jump.is_active && !self.dash.is_active
    }

    pub fn dash_possible(&self) -> bool {
        true
    }

    pub fn strike_possible(&self) -> bool {
        true
    }

    pub fn handle_jump(&mut self, transform: &mut Transform) {
        if Abilities::jump_possible(&self) {
            self.jump.is_active = true;
        }
    }

    pub fn handle_all(&mut self, transform: &mut Transform) {
        if self.jump.is_active {
            transform.translation.y += self.jump.current_power;
            self.jump.current_power -= GRAVITY_CONST;

            let jump_done: bool = Bounds::check_bounds_y(transform);

            if jump_done {
                self.jump.is_active = false;
                self.jump.current_power = self.jump.power;
            }
        }
    }
}

