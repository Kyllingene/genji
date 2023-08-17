//! Tools for loading, managing, and playing audio.
//!
//! Genji uses [`kira`] for managing
//! audio.
//!
//! ```ignore
//! # use genji::prelude::*;
//!
//! struct Hp(pub u32);
//! struct Death(pub Sound);
//!
//! // In init...
//! # fn dummy1(world: mut World, audio: mut Audio, sound_data: &'static [u8]) {
//! let death_sound = Audio::sound(sound_data, SoundSettings::default()).unwrap();
//! world.spawn((
//!     Hp(20),
//!     Death(death_sound),
//! ));
//! # }
//!
//! // In onloop...
//! # fn dummy2(world: World, audio: mut Audio) {
//! for (id, (hp, sound)) in world.query::<(&Hp, &Death)>() {
//!     if hp.0 == 0 {
//!         audio.play(sound.0);
//!     }
//! }
//! # }
//! ```

use std::{fmt::Debug, io::Cursor, path::Path};

use kira::{
    manager::{AudioManager, AudioManagerSettings},
    sound::{
        streaming::{StreamingSoundData, StreamingSoundHandle},
        FromFileError, SoundData,
    },
};

pub use kira::{
    sound::{
        static_sound::{StaticSoundData as Sound, StaticSoundSettings as SoundSettings},
        streaming::StreamingSoundSettings as MusicSettings,
    },
    *,
};

use crate::store::Store;

pub type Music = StreamingSoundData<FromFileError>;
pub type MusicHandle = StreamingSoundHandle<FromFileError>;

/// A way to store and access
/// [`Sound`]s
/// via human-friendly names.
pub type SoundStore = Store<Sound>;

/// A way to store and access
/// [`Music`]s
/// via human-friendly names.
pub type MusicStore = Store<Music>;

/// The interface for creating and playing audio.
///
/// ```ignore
/// # use genji::prelude::*;
///
/// struct Hp(pub u32);
/// struct Death(pub Sound);
///
/// // In init...
/// # fn dummy1(world: mut World, audio: mut Audio) {
/// let death_sound = Audio::sound(sound_data, SoundSettings::default()).unwrap();
/// world.spawn((
///     Hp(20),
///     Death(death_sound),
/// ));
/// # }
///
/// // In onloop...
/// # fn dummy2(world: mut World, audio: mut Audio) {
/// for (id, (hp, sound)) in world.query::<(&Hp, &Death)>() {
///     if hp.0 == 0 {
///         audio.play(sound.0);
///     }
/// }
/// # }
/// ```
pub struct Audio(AudioManager);

impl Audio {
    pub fn new() -> Self {
        Self(
            AudioManager::new(AudioManagerSettings::default()).expect("failed to initialize audio"),
        )
    }

    /// Plays a [`Sound`] or [`Music`]
    pub fn play<S: SoundData>(&mut self, sound: S)
    where
        <S as SoundData>::Error: Debug,
    {
        if let Err(e) = self.0.play(sound) {
            eprintln!("failed to play sound: {e:?}");
        }
    }

    /// Creates [`Sound`] (short-lived audio) from static data.
    pub fn sound(data: &'static [u8], settings: SoundSettings) -> Option<Sound> {
        Sound::from_cursor(Cursor::new(data), settings).ok()
    }

    /// Creates [`Sound`] (short-lived audio) from a file.
    pub fn sound_from_file<P: AsRef<Path>>(path: P, settings: SoundSettings) -> Option<Sound> {
        Sound::from_file(path, settings).ok()
    }

    /// Creates [`Music`] (streamable audio) from static data.
    pub fn music(data: &'static [u8], settings: MusicSettings) -> Option<Music> {
        Music::from_cursor(Cursor::new(data), settings).ok()
    }

    /// Creates [`Music`] (streamable audio) from a file.
    pub fn music_from_file<P: AsRef<Path>>(path: P, settings: MusicSettings) -> Option<Music> {
        Music::from_file(path, settings).ok()
    }
}

impl Default for Audio {
    fn default() -> Self {
        Self::new()
    }
}
