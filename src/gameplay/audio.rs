use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioChannel, AudioControl, AudioPlugin, AudioSource};
use rand::{thread_rng, Rng};
use crate::AppState;

pub struct OwnAudioPlugin;

pub struct AudioAssets {
    battle_background: Handle<AudioSource>,
}

fn load_audio(mut commands: Commands, assets: Res<AssetServer>) {
    commands.insert_resource(AudioAssets {
        battle_background: assets.load("audio/amygdala.ogg"),
    });
}

fn start_background_audio(audio_asset: Res<AudioAssets>, audio: Res<Audio>) {
    audio.play(audio_asset.battle_background.clone())
        .with_volume(1.0);
}

impl Plugin for OwnAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(AudioPlugin)
            .add_startup_system(load_audio)
            .add_system_set(SystemSet::on_enter(AppState::InGame));
                // .with_system(start_background_audio));
    }
}

