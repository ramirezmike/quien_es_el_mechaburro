use bevy::{ecs::system::SystemParam, prelude::*};
use bevy_kira_audio::{AudioApp, AudioChannel, AudioPlugin, AudioSource, AudioControl};
use std::marker::PhantomData;

pub struct GameAudioPlugin;
impl Plugin for GameAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_audio_channel::<MusicChannel>()
            .add_audio_channel::<SoundChannel>()
            .add_plugin(AudioPlugin);
    }
}

#[derive(Resource)]
pub struct MusicChannel;
#[derive(Resource)]
pub struct SoundChannel;

#[derive(SystemParam)]
pub struct GameAudio<'w, 's> {
    music_channel: Res<'w, AudioChannel<MusicChannel>>,
    sound_channel: Res<'w, AudioChannel<SoundChannel>>,

    #[system_param(ignore)]
    phantom: PhantomData<&'s ()>,
}

impl<'w, 's> GameAudio<'w, 's> {
    pub fn play_bgm(&mut self, handle: &Handle<AudioSource>) {
        self.music_channel.stop();
        self.music_channel.set_volume(0.5);
        self.music_channel.play(handle.clone()).looped();
    }

    pub fn play_sfx(&mut self, handle: &Handle<AudioSource>) {
        self.sound_channel.set_volume(0.5);
        self.sound_channel.play(handle.clone());
    }
}
