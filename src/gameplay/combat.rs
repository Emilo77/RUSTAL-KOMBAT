use std::borrow::BorrowMut;
use bevy::prelude::*;
use crate::gameplay::{Bounds, Player, PlayerNum, PlayerSide};

pub struct CombatPlugin;

pub struct PowerupPlugin;

const GRAVITY_CONST: f32 = 0.2;

//Dash
const DASH_COOLDOWN: f32 = 1.0;
const DASH_SPEED_BONUS: f32 = 3.0;
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
    side: PlayerSide,
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
                side: PlayerSide::Left,
            },
            strike: Strike {
                cooldown: STRIKE_COOLDOWN,
                damage: STRIKE_DAMAGE,
                is_active: false,
            },
        }
    }

    pub fn walking_possible(&self) -> bool {
        !self.dash.is_active
    }

    pub fn jump_possible(&self) -> bool {
        !self.dash.is_active
    }

    pub fn dash_possible(&self) -> bool {
        true
    }

    pub fn strike_possible(&self) -> bool {
        true
    }

    pub fn dash_active(&self) -> bool {
        self.dash.is_active
    }

    pub fn handle_jump(&mut self, transform: &mut Transform) {
        if Abilities::jump_possible(&self) {
            self.jump.is_active = true;
        }
    }

    pub fn handle_dash(&mut self, transform: &mut Transform, side: PlayerSide) {
        if Abilities::jump_possible(&self) {
            self.dash.is_active = true;
            self.dash.side = side;
        }
    }

    pub fn handle_all(&mut self, player: &mut Player, transform: &mut Transform) {
        if self.jump.is_active && !self.dash.is_active {
            transform.translation.y += self.jump.current_power;
            self.jump.current_power -= GRAVITY_CONST;

            let jump_done: bool = Bounds::check_bounds_y(transform);

            if jump_done {
                self.jump.is_active = false;
                self.jump.current_power = self.jump.power;
            }
        }
        if self.dash.is_active {
            match self.dash.side {
                PlayerSide::Left => {
                    transform.translation.x -= self.dash.speed_bonus * player.speed;
                }
                PlayerSide::Right => {
                    transform.translation.x += self.dash.speed_bonus * player.speed;
                }
            }
            self.dash.duration -= 0.01;
            if self.dash.duration < 0.0 {
                self.dash.is_active = false;
                self.dash.duration = DASH_DURATION;
            }
        }
    }
}

pub fn movement(mut player_query: Query<(&mut Player, &mut Transform, &mut Abilities)>,
                keyboard_input: Res<Input<KeyCode>>) {
    for (mut player, mut transform, mut abilities) in player_query.borrow_mut() {
        if !Abilities::dash_active(&abilities) {
            if keyboard_input.pressed(player.controls.left) {
                transform.translation.x -= player.speed;
                player.side = PlayerSide::Left;
            }
            if keyboard_input.pressed(player.controls.right) {
                transform.translation.x += player.speed;
                player.side = PlayerSide::Right;
            }
            Bounds::check_bounds_x(&mut transform);
        }
    }
}

pub fn jumping(mut player_query: Query<(&mut Player, &mut Transform, &mut Abilities)>,
               keyboard_input: Res<Input<KeyCode>>) {
    for (mut player, mut transform, mut abilities) in player_query.borrow_mut() {

        if keyboard_input.just_pressed(player.controls.jump) {
            Abilities::handle_jump(&mut abilities, &mut transform);
        }
    }
}

pub fn dash_system(mut player_query: Query<(&mut Player, &mut Transform, &mut Abilities)>,
                   keyboard_input: Res<Input<KeyCode>>) {
    for (mut player, mut transform, mut abilities) in player_query.borrow_mut() {

        if keyboard_input.just_pressed(player.controls.dash) {
            Abilities::handle_dash(&mut abilities, &mut transform, player.side);
        }
    }
}

pub fn overall_combat(mut player_query: Query<(&mut Player, &mut Transform, &mut Abilities)>) {
    for (mut player, mut transform, mut abilities) in player_query.borrow_mut() {
        Abilities::handle_all(&mut abilities, &mut player, &mut transform);
    }
}

pub fn kill(mut player_query: Query<(&mut Player)>,
                   keyboard_input: Res<Input<KeyCode>>) {
    for mut player in player_query.borrow_mut() {
        if keyboard_input.just_pressed(KeyCode::L) {
            player.hp -= 200.0
        }
    }
}