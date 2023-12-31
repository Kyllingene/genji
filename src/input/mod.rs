//! Keyboard input.
//!
//! Genji uses an array of booleans to store key state,
//! and exposes that through a wrapper struct to make
//! indexing more convenient. Both keyboard keys and
//! mouse buttons are stored in the same
//! [`Key`] enum.
//!
//! ```
//! # use genji::input::{Key, Keys};
//!
//! let mut keys = Keys::new();
//!
//! assert!(!keys[Key::Space]);
//!
//! # keys[Key::Space] = true;
//! // You press the spacebar...
//! assert!(keys[Key::Space]);
//! ```

use std::ops::{Add, Index, IndexMut, Sub};

use glium::glutin::event::VirtualKeyCode;

const KEYS_NUM: usize = 87;

/// A set of keys. Get a keys state with `keys[key]`.
///
/// ```
/// # use genji::input::{Key, Keys};
///
/// let mut keys = Keys::new();
///
/// assert!(!keys[Key::Space]);
///
/// # keys[Key::Space] = true;
/// // You press the spacebar...
/// assert!(keys[Key::Space]);
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Keys([bool; KEYS_NUM]);

impl Keys {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for Keys {
    fn default() -> Self {
        Self([false; KEYS_NUM])
    }
}

impl Index<Key> for Keys {
    type Output = bool;

    fn index(&self, index: Key) -> &Self::Output {
        &self.0[index as usize]
    }
}

impl IndexMut<Key> for Keys {
    fn index_mut(&mut self, index: Key) -> &mut Self::Output {
        &mut self.0[index as usize]
    }
}

impl<T: Into<usize>> Index<T> for Keys {
    type Output = bool;

    fn index(&self, index: T) -> &Self::Output {
        &self.0[index.into()]
    }
}

impl<T: Into<usize>> IndexMut<T> for Keys {
    fn index_mut(&mut self, index: T) -> &mut Self::Output {
        &mut self.0[index.into()]
    }
}

/// A key. Corresponds to a number (0-86).
///
/// ```
/// # use genji::input::{Key, Keys};
///
/// let mut keys = Keys::new();
///
/// assert!(!keys[Key::Space]);
///
/// # keys[Key::Space] = true;
/// // You press the spacebar...
/// assert!(keys[Key::Space]);
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(usize)]
pub enum Key {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,

    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,

    Up,
    Left,
    Down,
    Right,

    Tab,
    Shift,
    RShift,
    Caps,
    Space,
    Esc,
    Ctrl,
    RCtrl,
    Alt,
    RAlt,
    Super,
    RSuper,
    Backspace,
    Enter,

    Backtick,
    Minus,
    Equals,
    Backslash,
    LBracket,
    RBracket,
    Semicolon,
    Apostrophe,
    Comma,
    Period,
    Slash,

    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,

    LClick,
    RClick,
    MClick,
    M1,
    M2,
    M3,
    M4,
}

impl Key {
    pub const fn from_keycode(code: u32) -> Option<Self> {
        Some(match code {
            30 => Key::A,
            48 => Key::B,
            46 => Key::C,
            32 => Key::D,
            18 => Key::E,
            33 => Key::F,
            34 => Key::G,
            35 => Key::H,
            23 => Key::I,
            36 => Key::J,
            37 => Key::K,
            38 => Key::L,
            50 => Key::M,
            49 => Key::N,
            24 => Key::O,
            25 => Key::P,
            16 => Key::Q,
            19 => Key::R,
            31 => Key::S,
            20 => Key::T,
            22 => Key::U,
            47 => Key::V,
            17 => Key::W,
            45 => Key::X,
            21 => Key::Y,
            44 => Key::Z,
            11 => Key::Zero,
            2 => Key::One,
            3 => Key::Two,
            4 => Key::Three,
            5 => Key::Four,
            6 => Key::Five,
            7 => Key::Six,
            8 => Key::Seven,
            9 => Key::Eight,
            10 => Key::Nine,
            200 => Key::Up,
            203 => Key::Left,
            206 => Key::Down,
            205 => Key::Right,
            15 => Key::Tab,
            42 => Key::Shift,
            54 => Key::RShift,
            58 => Key::Caps,
            57 => Key::Space,
            1 => Key::Esc,
            29 => Key::Ctrl,
            157 => Key::RCtrl,
            56 => Key::Alt,
            184 => Key::RAlt,
            219 => Key::Super,
            220 => Key::RSuper,
            14 => Key::Backspace,
            28 => Key::Enter,
            41 => Key::Backtick,
            12 => Key::Minus,
            13 => Key::Equals,
            43 => Key::Backslash,
            26 => Key::LBracket,
            27 => Key::RBracket,
            39 => Key::Semicolon,
            40 => Key::Apostrophe,
            51 => Key::Comma,
            52 => Key::Period,
            53 => Key::Slash,
            59 => Key::F1,
            60 => Key::F2,
            61 => Key::F3,
            62 => Key::F4,
            63 => Key::F5,
            64 => Key::F6,
            65 => Key::F7,
            66 => Key::F8,
            67 => Key::F9,
            68 => Key::F10,
            87 => Key::F11,
            88 => Key::F12,
            _ => return None,
        })
    }

    pub fn from_virtual(code: Option<VirtualKeyCode>) -> Option<Vec<Self>> {
        Some(match code? {
            VirtualKeyCode::Key1 => vec![Key::One],
            VirtualKeyCode::Key2 => vec![Key::Two],
            VirtualKeyCode::Key3 => vec![Key::Three],
            VirtualKeyCode::Key4 => vec![Key::Four],
            VirtualKeyCode::Key5 => vec![Key::Five],
            VirtualKeyCode::Key6 => vec![Key::Six],
            VirtualKeyCode::Key7 => vec![Key::Seven],
            VirtualKeyCode::Key8 => vec![Key::Eight],
            VirtualKeyCode::Key9 => vec![Key::Nine],
            VirtualKeyCode::Key0 => vec![Key::Zero],
            VirtualKeyCode::A => vec![Key::A],
            VirtualKeyCode::B => vec![Key::B],
            VirtualKeyCode::C => vec![Key::C],
            VirtualKeyCode::D => vec![Key::D],
            VirtualKeyCode::E => vec![Key::E],
            VirtualKeyCode::F => vec![Key::F],
            VirtualKeyCode::G => vec![Key::G],
            VirtualKeyCode::H => vec![Key::H],
            VirtualKeyCode::I => vec![Key::I],
            VirtualKeyCode::J => vec![Key::J],
            VirtualKeyCode::K => vec![Key::K],
            VirtualKeyCode::L => vec![Key::L],
            VirtualKeyCode::M => vec![Key::M],
            VirtualKeyCode::N => vec![Key::N],
            VirtualKeyCode::O => vec![Key::O],
            VirtualKeyCode::P => vec![Key::P],
            VirtualKeyCode::Q => vec![Key::Q],
            VirtualKeyCode::R => vec![Key::R],
            VirtualKeyCode::S => vec![Key::S],
            VirtualKeyCode::T => vec![Key::T],
            VirtualKeyCode::U => vec![Key::U],
            VirtualKeyCode::V => vec![Key::V],
            VirtualKeyCode::W => vec![Key::W],
            VirtualKeyCode::X => vec![Key::X],
            VirtualKeyCode::Y => vec![Key::Y],
            VirtualKeyCode::Z => vec![Key::Z],
            VirtualKeyCode::Escape => vec![Key::Esc],
            VirtualKeyCode::F1 => vec![Key::F1],
            VirtualKeyCode::F2 => vec![Key::F2],
            VirtualKeyCode::F3 => vec![Key::F3],
            VirtualKeyCode::F4 => vec![Key::F4],
            VirtualKeyCode::F5 => vec![Key::F5],
            VirtualKeyCode::F6 => vec![Key::F6],
            VirtualKeyCode::F7 => vec![Key::F7],
            VirtualKeyCode::F8 => vec![Key::F8],
            VirtualKeyCode::F9 => vec![Key::F9],
            VirtualKeyCode::F10 => vec![Key::F10],
            VirtualKeyCode::F11 => vec![Key::F11],
            VirtualKeyCode::F12 => vec![Key::F12],
            VirtualKeyCode::Left => vec![Key::Left],
            VirtualKeyCode::Up => vec![Key::Up],
            VirtualKeyCode::Right => vec![Key::Right],
            VirtualKeyCode::Down => vec![Key::Down],
            VirtualKeyCode::Return => vec![Key::Enter],
            VirtualKeyCode::Space => vec![Key::Space],
            VirtualKeyCode::Numpad0 => vec![Key::Zero],
            VirtualKeyCode::Numpad1 => vec![Key::One],
            VirtualKeyCode::Numpad2 => vec![Key::Two],
            VirtualKeyCode::Numpad3 => vec![Key::Three],
            VirtualKeyCode::Numpad4 => vec![Key::Four],
            VirtualKeyCode::Numpad5 => vec![Key::Five],
            VirtualKeyCode::Numpad6 => vec![Key::Six],
            VirtualKeyCode::Numpad7 => vec![Key::Seven],
            VirtualKeyCode::Numpad8 => vec![Key::Eight],
            VirtualKeyCode::Numpad9 => vec![Key::Nine],
            VirtualKeyCode::NumpadAdd => vec![Key::Equals],
            VirtualKeyCode::NumpadDivide => vec![Key::Slash],
            VirtualKeyCode::NumpadDecimal => vec![Key::Period],
            VirtualKeyCode::NumpadComma => vec![Key::Comma],
            VirtualKeyCode::NumpadEnter => vec![Key::Enter],
            VirtualKeyCode::NumpadEquals => vec![Key::Equals],
            VirtualKeyCode::NumpadMultiply => vec![Key::Eight],
            VirtualKeyCode::NumpadSubtract => vec![Key::Minus],
            VirtualKeyCode::Apostrophe => vec![Key::Apostrophe],
            VirtualKeyCode::Asterisk => vec![Key::Shift, Key::Eight],
            VirtualKeyCode::At => vec![Key::Shift, Key::Two],
            VirtualKeyCode::Backslash => vec![Key::Backslash],
            VirtualKeyCode::Colon => vec![Key::Shift, Key::Semicolon],
            VirtualKeyCode::Comma => vec![Key::Comma],
            VirtualKeyCode::Equals => vec![Key::Equals],
            VirtualKeyCode::Grave => vec![Key::Backtick],
            VirtualKeyCode::LAlt => vec![Key::Alt],
            VirtualKeyCode::LBracket => vec![Key::LBracket],
            VirtualKeyCode::LControl => vec![Key::Ctrl],
            VirtualKeyCode::LShift => vec![Key::Shift],
            VirtualKeyCode::LWin => vec![Key::Super],
            VirtualKeyCode::Minus => vec![Key::Minus],
            VirtualKeyCode::Period => vec![Key::Period],
            VirtualKeyCode::Plus => vec![Key::Shift, Key::Equals],
            VirtualKeyCode::RAlt => vec![Key::RAlt],
            VirtualKeyCode::RBracket => vec![Key::RBracket],
            VirtualKeyCode::RControl => vec![Key::RCtrl],
            VirtualKeyCode::RShift => vec![Key::RShift],
            VirtualKeyCode::RWin => vec![Key::RSuper],
            VirtualKeyCode::Semicolon => vec![Key::Semicolon],
            VirtualKeyCode::Slash => vec![Key::Slash],
            VirtualKeyCode::Tab => vec![Key::Tab],
            VirtualKeyCode::Underline => vec![Key::Shift, Key::Minus],
            VirtualKeyCode::Copy => vec![Key::Shift, Key::C],
            VirtualKeyCode::Paste => vec![Key::Shift, Key::V],
            VirtualKeyCode::Cut => vec![Key::Shift, Key::X],
            _ => return None,
        })
    }
}

impl<T> From<T> for Key
where
    usize: From<T>,
{
    fn from(value: T) -> Self {
        usize::from(value).into()
    }
}

impl<T: Into<Key>> Add<T> for Key {
    type Output = Self;

    fn add(self, rhs: T) -> Self::Output {
        (self as usize + rhs.into() as usize).into()
    }
}

impl<T: Into<Key>> Sub<T> for Key {
    type Output = Self;

    fn sub(self, rhs: T) -> Self::Output {
        (self as usize - rhs.into() as usize).into()
    }
}
