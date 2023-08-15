// use std::collections::HashMap;

// use once_cell::sync::Lazy;

// use crate::graphics::{Color, Sprite};
use crate::graphics::Color;
use crate::input::{self, Keys};
// use crate::helpers::hash;

#[derive(Debug, Clone)]
pub struct GameState<T> {
    pub title: String,
    pub width: u32,
    pub height: u32,

    pub clear_color: Option<Color>,

    pub state: T,
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
        }
    }
}
