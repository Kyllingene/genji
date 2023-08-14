use std::collections::HashMap;

use once_cell::sync::Lazy;

use crate::graphics::{Color, Sprite};
use crate::input::{self, Keys};
use crate::helpers::hash;

// TODO: does this even need to be thread-safe? if no, remove once_cell dep
pub(crate) static mut SPRITES_CHANGED: Lazy<bool> = Lazy::new(|| true);

#[derive(Debug, Clone)]
pub struct Sprites(HashMap<u64, Sprite>);

impl Sprites {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    /// Adds a new sprite, then returns the numerical (hashed) id.
    ///
    /// Note: String literals must be referenced (i.e. `&"foobar"`).
    pub fn add<I>(&mut self, id: &I, s: Sprite) -> u64 {
        let id = hash(id);
        self.0.insert(id, s);
        unsafe { *SPRITES_CHANGED = true };

        id
    }

    /// Gets a reference to a sprite by id.
    ///
    /// Note: String literals must be referenced (i.e. `&"foobar"`).
    pub fn get<I>(&self, id: &I) -> Option<&Sprite> {
        self.0.get(&hash(id))
    }

    /// Gets a mutable reference to a sprite by id.
    ///
    /// Note: String literals must be referenced (i.e. `&"foobar"`).
    pub fn get_mut<I>(&mut self, id: &I) -> Option<&mut Sprite> {
        let s = self.0.get_mut(&hash(id));
        if s.is_some() {
            unsafe { *SPRITES_CHANGED = true };
        }

        s
    }

    /// Removes a sprite by id, returning the sprite (if it exists).
    ///
    /// Note: String literals must be referenced (i.e. `&"foobar"`).
    pub fn remove<I>(&mut self, id: &I) -> Option<Sprite> {
        let s = self.0.remove(&hash(id));
        if s.is_some() {
            unsafe { *SPRITES_CHANGED = true };
        }

        s
    }
}

impl AsRef<HashMap<u64, Sprite>> for Sprites {
    fn as_ref(&self) -> &HashMap<u64, Sprite> {
        &self.0
    }
}

#[derive(Debug, Clone)]
pub struct GameState<T> {
    pub title: String,
    pub width: u32,
    pub height: u32,

    pub clear_color: Option<Color>,

    pub state: T,
    // pub sprites: HashMap<u64, Sprite>,
    pub keys: Keys,

    pub fps: u128,
    pub delta: u128,

    pub mouse_x: i32,
    pub mouse_y: i32,

    /// The change in the scroll wheel this frame.
    pub scroll: i32,

    /// Whether or not genji closes when the OS asks it to.
    pub close_on_request: bool,
    /// When `!close_on_request`, if genji has been asked to close.
    pub asked_to_close: bool,
    // pub(crate) sprites_changed: bool,
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
            // sprites: HashMap::new(),
            keys: input::keys(),

            fps: 1000 / fps,
            delta: 0,

            mouse_x: 0,
            mouse_y: 0,

            scroll: 0,

            close_on_request: false,
            asked_to_close: false,
            // sprites_changed: false,
        }
    }

    // /// Adds a new sprite, then returns the numerical (hashed) id.
    // ///
    // /// Note: String literals must be referenced (i.e. `&"foobar"`).
    // pub fn add_sprite<I>(&mut self, id: &I, s: Sprite) -> u64 {
    //     let id = hash(id);
    //     self.sprites.insert(id, s);
    //     self.sprites_changed = true;

    //     id
    // }

    // /// Gets a reference to a sprite by id.
    // ///
    // /// Note: String literals must be referenced (i.e. `&"foobar"`).
    // pub fn get_sprite<I>(&self, id: &I) -> Option<&Sprite> {
    //     self.sprites.get(&hash(id))
    // }

    // /// Gets a mutable reference to a sprite by id.
    // ///
    // /// Note: String literals must be referenced (i.e. `&"foobar"`).
    // pub fn get_sprite_mut<I>(&mut self, id: &I) -> Option<&mut Sprite> {
    //     let s = self.sprites.get_mut(&hash(id));
    //     if s.is_some() {
    //         self.sprites_changed = true;
    //     }

    //     s
    // }

    // /// Removes a sprite by id, returning the sprite (if it exists).
    // ///
    // /// Note: String literals must be referenced (i.e. `&"foobar"`).
    // pub fn remove_sprite<I>(&mut self, id: &I) -> Option<Sprite> {
    //     let s = self.sprites.remove(&hash(id));
    //     if s.is_some() {
    //         self.sprites_changed = true;
    //     }

    //     s
    // }
}

unsafe impl<T: Send> Send for GameState<T> {}
