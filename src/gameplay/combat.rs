use std::borrow::{BorrowMut};
use bevy::prelude::*;
use crate::gameplay::{AudioDashEvent, Bounds, Player, PLAYER_SIZE, PlayerNum, PlayerSide};

const GRAVITY_CONST: f32 = 0.2;

//Dash
const DASH_COOLDOWN: f32 = 10.0;
const DASH_SPEED_BONUS: f32 = 3.0;
const DASH_DURATION: f32 = 0.25;
const DASH_DAMAGE: i32 = 9;
//Jump
const JUMP_POWER: f32 = 13.0;

const PLAYER_HURTING_TIME: f32 = 10.0;
const HITBOX_RATIO: f32 = 0.6;

#[derive(Component)]
pub struct Dash {
    cooldown: f32,
    speed_bonus: f32,
    duration: f32,
    damage: i32,
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
pub struct Abilities {
    pub jump: Jump,
    pub dash: Dash,
    pub player_num: PlayerNum,
}

pub struct DashInformation {
    pub player_cords: Vec2,
    pub dash_active: bool,
}

impl Abilities {
    pub fn new(player_num: PlayerNum) -> Self {
        Abilities {
            jump: Jump {
                power: JUMP_POWER,
                current_power: JUMP_POWER,
                is_active: false,
            },
            dash: Dash {
                cooldown: 0.0,
                speed_bonus: DASH_SPEED_BONUS,
                duration: DASH_DURATION,
                damage: DASH_DAMAGE,
                is_active: false,
                side: PlayerSide::Left,
            },
            player_num,
        }
    }

    pub fn jump_possible(&self) -> bool {
        !self.dash.is_active
    }

    pub fn dash_active(&self) -> bool {
        self.dash.is_active
    }

    pub fn handle_jump(&mut self) {
        if Abilities::jump_possible(&self) {
            self.jump.is_active = true;
        }
    }

    pub fn handle_dash(&mut self, side: PlayerSide) -> bool {
        if self.dash.cooldown <= 0.0 {
            self.dash.is_active = true;
            self.dash.side = side;
            return true;
        }
        return false;
    }

    pub fn handle_hurting_cooldown(player: &mut Player) {
        if player.hurting > 0.0 {
            player.hurting -= 0.1;
        }
    }

    pub fn handle_dash_cooldown(&mut self) {
        if self.dash.cooldown > 0.0 {
            self.dash.cooldown -= 0.1;
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
                self.dash.cooldown = DASH_COOLDOWN;
            }
        }
    }

    pub fn give_dash_info(&self, transform: &Transform) -> DashInformation {
        DashInformation {
            player_cords: Vec2::new(transform.translation.x, transform.translation.y),
            dash_active: self.dash.is_active,
        }
    }
}

pub fn movement(mut player_query: Query<(&mut Player, &mut Transform, &mut Abilities)>,
                keyboard_input: Res<Input<KeyCode>>) {
    for (mut player, mut transform, abilities) in player_query.borrow_mut() {
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

pub fn jumping(mut player_query: Query<(&mut Player, &mut Abilities)>,
               keyboard_input: Res<Input<KeyCode>>) {
    for (player, mut abilities) in player_query.borrow_mut() {
        if keyboard_input.just_pressed(player.controls.jump) {
            Abilities::handle_jump(&mut abilities);
        }
    }
}

pub fn dashing(mut player_query: Query<(&mut Player, &mut Abilities)>,
               keyboard_input: Res<Input<KeyCode>>,
               mut audio_hit_event: EventWriter<AudioDashEvent>, ) {
    for (player, mut abilities) in player_query.borrow_mut() {
        if keyboard_input.just_pressed(player.controls.dash)
            && Abilities::handle_dash(&mut abilities, player.side) {
            audio_hit_event.send(AudioDashEvent)
        }
    }
}

pub fn overall_combat(mut player_query: Query<(&mut Player, &mut Transform, &mut Abilities)>) {
    let mut dash_info_p1: DashInformation = DashInformation {
        player_cords: Default::default(),
        dash_active: false,
    };
    let mut dash_info_p2: DashInformation = DashInformation {
        player_cords: Default::default(),
        dash_active: false,
    };

    let mut p1_should_get_dmg: bool = false;
    let mut p2_should_get_dmg: bool = false;

    for (mut player, mut transform, mut abilities) in player_query.borrow_mut() {

        Abilities::handle_hurting_cooldown(&mut player);
        Abilities::handle_dash_cooldown(&mut abilities);
        Abilities::handle_all(&mut abilities, &mut player, &mut transform);

        match player.num {
            PlayerNum::One => {
                dash_info_p1 = Abilities::give_dash_info(&abilities, &transform)
            }
            PlayerNum::Two => {
                dash_info_p2 = Abilities::give_dash_info(&abilities, &transform)
            }
        }
    }
    if dash_info_p1.dash_active
        && in_hitbox_range(dash_info_p1.player_cords, dash_info_p2.player_cords) {
        p2_should_get_dmg = true;
    }

    if dash_info_p2.dash_active
        && in_hitbox_range(dash_info_p2.player_cords, dash_info_p1.player_cords) {
        p1_should_get_dmg = true;
    }

    for (mut player, _transform, abilities) in player_query.borrow_mut() {
        if player.num == PlayerNum::One && p1_should_get_dmg {
            deal_damage(&mut player, abilities.dash.damage)
        }
        if player.num == PlayerNum::Two && p2_should_get_dmg {
            deal_damage(&mut player, abilities.dash.damage)
        }
    }
}

pub fn in_hitbox_range(pos1: Vec2, pos2: Vec2) -> bool {
    return (pos1.x >= pos2.x - HITBOX_RATIO * PLAYER_SIZE.x)
        && (pos1.x <= pos2.x + HITBOX_RATIO * PLAYER_SIZE.x)
        && (pos1.y >= pos2.y - HITBOX_RATIO * PLAYER_SIZE.y)
        && (pos1.y <= pos2.y + HITBOX_RATIO * PLAYER_SIZE.y);
}

pub fn deal_damage(player: &mut Player, damage: i32) {
    if player.hurting <= 0.0 {
        player.hurting = PLAYER_HURTING_TIME;
        player.hp -= damage;
    }
}
