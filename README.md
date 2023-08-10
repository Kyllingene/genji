# Genji Game Engine

Genji is my custom game engine written in Rust. It's

## Example usage

```rust
use genji::prelude::*;

// This line injects the genji code that runs your game.
// While this can go anywhere, it will disable LSP for
// that code segment, so this is a good place.
#[genji::init]
type GameState = GameState<i32>;

// This function initializes the game. Here is where you
// should do things like load assets (if you don't utilize
// `use_file<s>!`), construct levels, etc.
fn init() -> GameState {
    let mut state = State::new(
        // Your custom state (this can be anything, as long as it
        // has a static lifetime; so no non-static references, but
        // any normal type/struct is okay).
        0,
        // The title.
        "Genji Test",
        // The window width; defaults to 640.
        None,
        // The window height; defaults to 480.
        None,
        // The target FPS; None defaults to 30.
        None,
        // The clear color; None will disable screen clearing.
        Some(Color::default())
    );

    let circle = Sprite::circle(
        20, // radius
        
        // This is what holds most of the information about
        // your sprites: position, rotation, color, depth.
        //
        // Coordinates run from -400 to 400.
        SpriteData::new()
            .xy(-380, 0)
            .fill(true)
            .color(Color::new(255, 100, 50, 255))
    );

    state.add_sprite(&"circle", circle);
    state
}

// This is where the bulk of your code goes; it is run every frame (unfortunately,
// genji's logic is not yet framerate-independent).
fn onloop(state: &mut GameState) -> bool {
    // The current system requires you cloning the data you need from the
    // GameState *before* modifying your sprites; this is being worked on.
    let width = state.width;
    let keys = state.keys;

    if let Some(circle) = state.get_sprite_mut(&"circle") {
        // This returns a &mut SpriteData.
        let data = circle.sprite_data_mut();
        
        // GameState.keys is a wrapper around an array, where each index is a key
        // and each item is a boolean representing whether or not it is pressed.
        if keys[Key::Up] {
            data.y += 3;
            data.y %= height as i32;
        }

        if keys[Key::Left] {
            data.x -= 3;
            data.x %= width as i32;
        }

        if keys[Key::Down] {
            data.y -= 3;
            data.y %= height as i32;
        }

        if keys[Key::Right] {
            data.x += 3;
            data.x %= width as i32;
        }
    }

    // If you return false, the game will keep running. Returning true from this
    // function is how you tell genji to stop. However, if the OS asks the game
    // to close, it will run the close function (below) and close the window.
    // You can disable that via GameState.close_on_request and .asked_to_close.
    false
}

// This is your destructor. Use this to do things like write save files.
fn close(_state: State) {}
```
