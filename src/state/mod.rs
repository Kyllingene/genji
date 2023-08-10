use std::collections::HashMap;
use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

use crate::graphics::{Color, Sprite};
use crate::input::{self, Keys};

/// Hashes any type byte-by-byte.
///
/// The &str "abc" and the &[u8] [61, 62, 63] result in the same hash.
fn hash<T>(data: &T) -> u64 {
    let mut hasher = DefaultHasher::new();
    let bytes: &[u8] = unsafe {
        core::slice::from_raw_parts((data as *const T) as *const u8, ::core::mem::size_of::<T>())
    };
    for byte in bytes {
        byte.hash(&mut hasher);
    }

    hasher.finish()
}

pub struct GameState<T> {
    pub title: String,
    pub width: u32,
    pub height: u32,

    pub clear_color: Option<Color>,

    pub state: T,
    pub sprites: HashMap<u64, Sprite>,

    pub keys: Keys,

    pub fps: u128,
    pub delta: u128,

    /// Whether or not Genji closes when the OS asks it to.
    pub close_on_request: bool,
    /// If `!close_on_request`, if Genji has been asked to close.
    pub asked_to_close: bool,

    pub(crate) sprites_changed: bool,
}

impl<T> GameState<T> {
    /// Initiates genji's game state. Creates a new window.
    ///
    /// `width` and `height` may be None, defaulting to 640 and 480 respectively.
    /// `fps` defaults to 100.
    ///
    /// If `clear_color` is None, the screen is never cleared.
    pub fn new<S: ToString>(
        state: T,
        title: S,
        width: Option<u32>,
        height: Option<u32>,
        fps: Option<u128>,
        clear_color: Option<Color>,
    ) -> Self {
        let title = title.to_string();
        let width = width.unwrap_or(640);
        let height = height.unwrap_or(480);
        let fps = fps.unwrap_or(100);

        Self {
            title,
            width,
            height,

            clear_color,

            state,
            sprites: HashMap::new(),

            keys: input::keys(),

            fps: 1000 / fps,
            delta: 0,

            close_on_request: false,
            asked_to_close: false,

            sprites_changed: false,
        }
    }

    /// Adds a new sprite, then returns the numerical (hashed) id.
    ///
    /// Note: String literals must be referenced (i.e. `&"foobar"`).
    pub fn add_sprite<I>(&mut self, id: &I, s: Sprite) -> u64 {
        let id = hash(id);
        self.sprites.insert(id, s);
        self.sprites_changed = true;

        id
    }

    /// Gets a reference to a sprite by id.
    ///
    /// Note: String literals must be referenced (i.e. `&"foobar"`).
    pub fn get_sprite<I>(&self, id: &I) -> Option<&Sprite> {
        self.sprites.get(&hash(id))
    }

    /// Gets a mutable reference to a sprite by id.
    ///
    /// Note: String literals must be referenced (i.e. `&"foobar"`).
    pub fn get_sprite_mut<I>(&mut self, id: &I) -> Option<&mut Sprite> {
        let s = self.sprites.get_mut(&hash(id));
        if s.is_some() {
            self.sprites_changed = true;
        }

        s
    }

    /// Removes a sprite by id, returning the sprite (if it exists).
    ///
    /// Note: String literals must be referenced (i.e. `&"foobar"`).
    pub fn remove_sprite<I>(&mut self, id: &I) -> Option<Sprite> {
        let s = self.sprites.remove(&hash(id));
        if s.is_some() {
            self.sprites_changed = true;
        }

        s
    }
}

unsafe impl<T: Send> Send for GameState<T> {}
