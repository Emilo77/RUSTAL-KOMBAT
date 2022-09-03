use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioControl, AudioPlugin, AudioSource};
use crate::AppState;

pub struct OwnAudioPlugin;

pub struct AudioDashEvent;

const BASIC_VOLUME: f64 = 0.4;
const BATTLE_BACKGROUND_VOLUME: f64 = 0.15;

impl Plugin for OwnAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(AudioPlugin)
            .add_startup_system_to_stage(StartupStage::PreStartup, load_audio)

            .add_system(play_swordhit_sfx)
            .add_event::<AudioDashEvent>()

            .add_system_set(SystemSet::on_enter(AppState::StartingMenu)
                .with_system(start_menu_music))
            .add_system_set(SystemSet::on_exit(AppState::StartingMenu)
                .with_system(stop_music))

            .add_system_set(SystemSet::on_enter(AppState::InGame)
                .with_system(start_gameplay_music))
            .add_system_set(SystemSet::on_exit(AppState::InGame)
                .with_system(stop_music))

            .add_system_set(SystemSet::on_enter(AppState::EndMenuWinP1)
                .with_system(start_ending_music))
            .add_system_set(SystemSet::on_exit(AppState::EndMenuWinP1)
                .with_system(stop_music))

            .add_system_set(SystemSet::on_enter(AppState::EndMenuWinP2)
                .with_system(start_ending_music))
            .add_system_set(SystemSet::on_exit(AppState::EndMenuWinP2)
                .with_system(stop_music))

            .add_system_set(SystemSet::on_enter(AppState::EndMenuDraw)
                .with_system(start_ending_music))
            .add_system_set(SystemSet::on_exit(AppState::EndMenuDraw)
                .with_system(stop_music));
    }
}

pub struct AudioAssets {
    battle_background: Handle<AudioSource>,
    starting_background: Handle<AudioSource>,
    ending_sound: Handle<AudioSource>,
    dash: Handle<AudioSource>,
}

fn load_audio(mut commands: Commands, assets: Res<AssetServer>) {
    commands.insert_resource(AudioAssets {
        starting_background: assets.load("audio/b_nice_2_me.ogg"),
        battle_background: assets.load("audio/not_dead_yet.ogg"),
        ending_sound: assets.load("audio/ending_sound.ogg"),
        dash: assets.load("audio/sword_slice.ogg"),
    });
}

fn start_menu_music(audio_asset: Res<AudioAssets>, audio: Res<Audio>) {
    audio.play(audio_asset.starting_background.clone())
        .with_volume(BASIC_VOLUME);
}

fn start_gameplay_music(audio_asset: Res<AudioAssets>, audio: Res<Audio>) {
    audio.play(audio_asset.battle_background.clone())
        .with_volume(BATTLE_BACKGROUND_VOLUME);
}

fn start_ending_music(audio_asset: Res<AudioAssets>, audio: Res<Audio>) {
    audio.play(audio_asset.ending_sound.clone())
        .with_volume(BASIC_VOLUME);
}

fn stop_music(audio: Res<Audio>) {
    audio.stop();
}

pub fn play_swordhit_sfx(
    audio: Res<Audio>,
    audio_state: Res<AudioAssets>,
    mut audio_event: EventReader<AudioDashEvent>,
) {
    audio_event.iter().for_each(|_| {
        audio.play(audio_state.dash.clone()).with_volume(BASIC_VOLUME);
    });
}


