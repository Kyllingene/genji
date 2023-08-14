# Genji

Genji is a custom game engine written in Rust. It's designed to provide an experience similar to most popular game engines, where your code is run inside a framework that handles the heavy work for you. However, it's technically just a gamedev library that uses some macro magic to give the illusion of an engine.

Currently, it is far from optimized. It also doesn't have a very good sprite system, though it supports several primitives, text, and images. It's very rough around the edges. Top priority is adding sound, then fixing the sprite model.

## Building

Since games can be performance-sensitive, it's often important to have optimizations, even in dev builds. However, building in release mode will cause your compile times to drastically increase, harming the development cycle. A good compromise is to build your dependencies (including Genji) optimized, since they won't be re-compiled every time, while leaving your actual game code unoptimized. This can be achieved by putting this into your `Cargo.toml`:

```toml
[profile.dev.package."*"]
opt-level = 3
```

## Assets

For images and fonts (the only two assets currently required), Genji supports both loading from bytes and loading from a file. The method I would recommend for small games (the only ones this can really do) is using the provided `use_file!` and `use_files!` macros inside of a module to provide namespaced access to pre-loaded assets.

Bundles and spritemaps are parts of an ideal future, but it may not come to fruition.

## Example usage

```rust
use genji::prelude::*;

// This line injects the genji code that runs your game.
// While this can go anywhere, it will disable LSP for
// that code segment, so this is a good place.
#[genji::init]
type State = GameState<i32>;

// This function initializes the game. Here is where you
// should do things like load assets (if you don't utilize
// `use_file<s>!`), construct levels, etc.
fn init() -> (State, Sprites) {
    let mut sprites = Sprites::new();
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

    sprites.add(&"circle", circle);
    (state, sprites)
}

// This is where the bulk of your code goes; it is run every frame (unfortunately,
// genji's logic is not yet framerate-independent).
fn onloop(state: &mut State, sprites: &mut Sprites) -> bool {
    // GameState.keys is a wrapper around an array, where each index is a key
    // and each item is a boolean representing whether or not it is pressed.
    if state.keys[Key::Esc] {
        return true;
    }

    if let Some(circle) = sprites.get_mut(&"circle") {
        // This returns a &mut SpriteData.
        let data = circle.sprite_data_mut();

        data.x += 3;
        data.x %= state.width as i32;
    }

    // If you return false, the game will keep running. Returning true from this
    // function is how you tell genji to stop. However, if the OS asks the game
    // to close, it will run the close function (below) and close the window.
    // You can disable that via GameState.close_on_request and .asked_to_close.
    false
}

// This is your destructor. Use this to do things like write save files.
fn close(_state: State, _sprites: Sprites) {}
```

## FFI

While I highly doubt this is in Genji's future, it is technically possible to make a wrapper around Genji for another systems programming language, such as C. To do this, you would have to find your own way to run `genji::main` (with the requisite functions), as that is what does all the work for the engine. You would also need bindings to the library functions, most of which are methods on `GameState` or `Sprite`.

## Contributing

At the moment, Genji is so far from its vision that I will not be accepting contributions. My reasons are several: for API changes, they may not line up with my goal for Genji, or could get in the way down the road; for optimizations, they may end up impeding API changes or end up getting removed entirely. Bugs should be reported via an [issue](https://github.com/kyllingene/genji), as they may hint at an API change that needs to be made. Suggestions are always welcome (through the issues page), but there is no guarantee I will follow them.