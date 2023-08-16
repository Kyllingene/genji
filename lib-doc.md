# Genji

Genji is a custom game engine written in Rust. It's designed to provide an experience similar to most popular game engines, where your code is run inside a framework that handles the heavy work for you. However, it's technically just a gamedev library that uses some macro magic to give the illusion of an engine.

Currently, it is far from optimized. It's very rough around the edges, and in dire need of improvement. The best way to help is to use it (even for toy projects), and give feedback on the parts that suck.

## Building

Since games can be performance-sensitive, it's often important to have optimizations, even in dev builds. However, building in release mode will cause your compile times to drastically increase, harming the development cycle. A good compromise is to build your dependencies (including genji) optimized, since they won't be re-compiled every time, while leaving your actual game code unoptimized. This can be achieved by putting this into your `Cargo.toml`:

```toml
[profile.dev.package."*"]
opt-level = 3
```

## Assets

For images and fonts (the only two assets currently required), genji supports both loading from bytes and loading from a file. The method I would recommend for small games (the only ones this can really do) is using the provided `use_file!` and `use_files!` macros inside of a module to provide namespaced access to pre-loaded assets.

Bundles and spritemaps are parts of an ideal future, but it may not come to fruition.

## Example usage

```ignore
use genji::prelude::*;

// This line injects the genji code that runs your game.
// While this can go anywhere, it will disable LSP for
// that code segment, so this is a good place.
#[genji::init]
type State = GameState<Entity>;

// This function initializes the game. Here is where you
// should do things like load assets (if you don't utilize
// `use_file<s>!`), construct levels, etc.
fn init() -> (State, World) {
    let mut world = World::new();

    // Genji uses an ECS called [hecs](https://docs.rs/hecs/0.10.3/hecs/index.html).
    // This tuple is an entity in your game, and it has certain bits of functionality.
    // Note that when you omit details from a sprite (such as rotation), they use
    // default values. Position is the exception: no sprite without a Position is drawn.
    // These can be in any order.
    let circle = (
        sprite::circle(20),
        Fill(false),
        Color::default().r(0),
        // Coordinates in genji are based on the height of your window; in
        // the vertical dimension, coordinates are always -400 to 400. However,
        // horizontal coordinates can go higher if the window is wider than it
        // is high.
        Position(20, 20),
        Depth(5),
    );

    // This returns an Entity, which is technically just an ID for an entity.
    let circle = world.spawn(circle);

    let state = State::new(
        // Your custom state (this can be anything, as long as it
        // has a static lifetime; so no non-static references, but
        // any normal type/struct is okay).
        circle,
        // The title.
        "Genji Test",
        // The window width; defaults to 640.
        None,
        // The window height; defaults to 480.
        None,
        // The target FPS; None defaults to 100.
        None,
        // The clear color; None will disable screen clearing.
        Some(Color::default()),
    );
    (state, world)
}

// This is where the bulk of your code goes; it is run every frame (unfortunately,
// genji's logic is not yet framerate-independent).
fn onloop(state: &mut State, world: &mut World) -> bool {
    if state.keys[Key::Esc] {
        return true;
    }

    if let Ok((circle, position, fill)) =
        world.query_one_mut::<(&mut sprite::Circle, &mut Position, &mut Fill)>(state.state)
    {
        position.0 = state.mouse_x;
        position.1 = state.mouse_y;

        // GameState.keys is a wrapper around an array, where each index is a key
        // and each item is a boolean representing whether or not it is pressed.
        if state.keys[Key::RClick] {
            circle.r = 25;
        } else {
            circle.r = 20;
        }

        **fill = state.keys[Key::LClick];
    }

    // If you return false, the game will keep running. Returning true from this
    // function is how you tell genji to stop. However, if the OS asks the game
    // to close, it will run the close function (below) and close the window.
    // You can disable that via GameState.close_on_request and .asked_to_close.
    false
}

// This is your destructor. Use this to do things like write save files.
fn close(_state: State, _world: World) {}
```

## FFI

While I highly doubt this is in genji's future, it is technically possible to make a wrapper around genji for another systems programming language, such as C. To do this, you would have to make your own way to run `genji::main` (with the user functions), as that is what does all the work for the engine. You would also need bindings to the library functions, including all exposed dependency features (i.e. all of `hecs`).

## Contributing

At the moment, genji is so far from its vision that I will not be accepting contributions. My reasons are several: for API changes, they may not line up with my goal for genji, or could get in the way down the road; for optimizations, they may end up impeding API changes or end up getting removed entirely. Bugs should be reported via an [issue](https://github.com/kyllingene/genji) (they may also hint at an API change that needs to be made). Suggestions are always welcome (through the issues page), but there is no guarantee I will follow them.
