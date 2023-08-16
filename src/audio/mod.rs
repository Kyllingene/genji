//! Tools for loading, managing, and playing audio.
//!
//! Genji uses [`kira`](https://docs.rs/kira/0.8.4/kira) for managing
//! audio.

use std::{path::Path, io::Cursor, collections::HashMap};

pub use kira::{
    *,
    sound::static_sound::{
        StaticSoundData as Sound,
        StaticSoundSettings as SoundSettings,
    },
};

use kira::manager::{
    AudioManager, AudioManagerSettings,
    backend::DefaultBackend,
};

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
pub struct Audio(AudioManager<DefaultBackend>);

impl Audio {
    pub fn new() -> Self {
        Self (
            AudioManager::new(
                AudioManagerSettings::default()
            ).expect("failed to initialize audio")
        )
    }

    pub fn play(&mut self, sound: Sound) {
        if let Err(e) = self.0.play(sound) {
            eprintln!("failed to play sound: {e:?}");
        }
    }

    pub fn sound(data: &'static [u8], settings: SoundSettings) -> Option<Sound> {
        Sound::from_cursor(Cursor::new(data), settings).ok()
    }

    pub fn sound_from_file<P: AsRef<Path>>(path: P, settings: SoundSettings) -> Option<Sound> {
        Sound::from_file(path, settings).ok()
    }
}

impl Default for Audio {
    fn default() -> Self {
        Self::new()
    }
}

/// A way to store and access
/// [`Sound`](../struct.Audio.)s
/// via human-friendly names.
#[derive(Clone, Debug, Default)]
pub struct SoundStore(HashMap<String, Sound>);

impl SoundStore {
    pub fn new() -> Self {
        Self::default()
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
    pub fn with<S: ToString>(mut self, id: S, sound: Sound) -> Self {
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
    pub fn add<S: ToString>(&mut self, id: S, sound: Sound) {
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
    pub fn get<S: ToString>(&self, id: S) -> Option<Sound> {
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
    pub fn remove<S: ToString>(&mut self, id: S) -> Option<Sound> {
        self.0.remove(&id.to_string())
    }
}

