use bevy::prelude::*;

pub struct PlayerPlugin;

struct ScoreBoard {
    score1: usize,
    score2: usize,
}

#[derive(Component)]
struct Player {
    hp: u8,
    velocity: f32,
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        //todo!()
    }
}


