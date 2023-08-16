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
//! # fn dummy1(world: mut World, audio: mut Audio) {
//! let death_sound = Audio::sound(sound_data, SoundSettings::default()).unwrap();
//! world.spawn((
//!     Hp(20),
//!     Death(death_sound),
//! ));
//! # }
//! 
//! // In onloop...
//! # fn dummy2(world: mut World, audio: mut Audio) {
//! for (id, (hp, sound)) in world.query::<(&Hp, &Death)>() {
//!     if hp.0 == 0 {
//!         audio.play(sound.0);
//!     }
//! }
//! # }
//! ```

use std::{path::Path, io::Cursor, collections::HashMap, fmt::Debug};

use kira::{sound::{streaming::{
    StreamingSoundData,
    StreamingSoundHandle,
}, FromFileError, SoundData}, manager::{AudioManager, AudioManagerSettings}};
pub use kira::{
    *,
    sound::{
        static_sound::{
            StaticSoundData as Sound,
            StaticSoundSettings as SoundSettings,
        },
        streaming::StreamingSoundSettings as MusicSettings,
    }
};

pub type Music = StreamingSoundData<FromFileError>;
pub type MusicHandle = StreamingSoundHandle<FromFileError>;

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
        Self (
            AudioManager::new(
                AudioManagerSettings::default()
            ).expect("failed to initialize audio")
        )
    }

    /// Plays a [`Sound`] or [`Music`]
    pub fn play<S: SoundData>(&mut self, sound: S) where <S as SoundData>::Error: Debug {
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

/// A way to store and access [`Sound`]s and [`Music`]s
/// via human-friendly names.
#[derive(Clone, Debug)]
pub struct AudioStore<S: SoundData + Clone>(HashMap<String, S>);

impl<S: SoundData + Clone> AudioStore<S> {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    /// Store a sound in a builder pattern.
    /// 
    /// ```
    /// # use genji::ecs::{Sound, SoundStore};
    /// # let store = SoundStore::new();
    /// # let sound = unsafe { std::mem::zeroed() };
    /// 
    /// let store = store.with("sound", sound);
    /// ```
    pub fn with<I: ToString>(mut self, id: I, sound: S) -> Self {
        self.0.insert(id.to_string(), sound);
        self
    }

    /// Store a sound.
    /// 
    /// ```
    /// # use genji::ecs::{Sound, SoundStore};
    /// # let mut store = SoundStore::new();
    /// # let sound = unsafe { std::mem::zeroed() };
    /// 
    /// store.add("sound", sound);
    /// ```
    pub fn add<I: ToString>(&mut self, id: I, sound: S) {
        self.0.insert(id.to_string(), sound);
    }

    /// Gets a sound, returning it if it exists.
    /// 
    /// ```
    /// # use genji::ecs::{Sound, SoundStore};
    /// # let mut store = SoundStore::new();
    /// # let sound = unsafe { std::mem::zeroed() };
    /// store.add("sound", sound);
    /// 
    /// assert!(store.get("sound").is_some());
    /// ```
    pub fn get<I: ToString>(&self, id: I) -> Option<S> {
        self.0.get(&id.to_string()).cloned()
    }

    /// Remove a sound, returning it if it exists.
    /// 
    /// ```
    /// # use genji::ecs::{Sound, SoundStore};
    /// # let mut store = SoundStore::new();
    /// # let sound = unsafe { std::mem::zeroed() };
    /// store.add("sound", sound);
    /// 
    /// assert!(store.remove("sound").is_some());
    /// assert!(store.remove("sound").is_none());
    /// ```
    pub fn remove<I: ToString>(&mut self, id: I) -> Option<S> {
        self.0.remove(&id.to_string())
    }
}

pub type SoundStore = AudioStore<Sound>;
pub type MusicStore = AudioStore<Music>;
