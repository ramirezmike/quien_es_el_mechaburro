use bevy::{ecs::system::SystemParam, prelude::*};
use bevy_kira_audio::{Audio, AudioChannel, AudioPlugin, AudioSource};
use std::marker::PhantomData;

pub struct GameAudioPlugin;
impl Plugin for GameAudioPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AudioState::new())
            .add_plugin(AudioPlugin);
    }
}

pub struct ChannelAudioState {
    channel: AudioChannel,
    volume: f32,
}

impl ChannelAudioState {
    fn new(audio_path: &str) -> Self {
        ChannelAudioState {
            channel: AudioChannel::new(audio_path.to_owned()),
            volume: 0.6,
        }
    }
}

#[derive(SystemParam)]
pub struct GameAudio<'w, 's> {
    audio: Res<'w, Audio>,
    audio_state: ResMut<'w, AudioState>,

    #[system_param(ignore)]
    phantom: PhantomData<&'s ()>,
}

impl<'w, 's> GameAudio<'w, 's> {
    pub fn play_bgm(&mut self, handle: &Handle<AudioSource>) {
        self.audio.set_volume_in_channel(
            self.audio_state.bgm_channel.volume,
            &self.audio_state.bgm_channel.channel,
        );
        self.audio
            .play_looped_in_channel(handle.clone(), &self.audio_state.bgm_channel.channel);
    }

    pub fn play_sfx(&mut self, handle: &Handle<AudioSource>) {
        self.audio.set_volume_in_channel(
            self.audio_state.sfx_channel.volume,
            &self.audio_state.sfx_channel.channel,
        );
        self.audio
            .play_in_channel(handle.clone(), &self.audio_state.sfx_channel.channel);
    }
}

pub struct AudioState {
    pub sfx_channel: ChannelAudioState,
    pub bgm_channel: ChannelAudioState,
}

impl AudioState {
    pub fn new() -> AudioState {
        let sfx_channel = ChannelAudioState::new("sound");
        let bgm_channel = ChannelAudioState::new("music");

        AudioState {
            sfx_channel,
            bgm_channel,
        }
    }
}
