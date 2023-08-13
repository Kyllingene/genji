#![doc = include_str!("../README.md")]

use std::thread;
use std::time::{Duration, Instant};

pub use genji_macros::init;

pub mod graphics;
pub mod input;
pub mod state;

use input::Key;

use glium::{glutin, Surface};
use state::{GameState, Sprites, SPRITES_CHANGED};

pub mod prelude;

mod helpers;
use helpers::gl2gj;

/// Runs the engine code for genji. Automatically run
/// via `genji::init`, so please don't do this manually.
pub fn main<T: 'static>(
    init: impl FnOnce() -> (GameState<T>, Sprites) + 'static,
    onloop: impl Fn(&mut GameState<T>, &mut Sprites) -> bool + 'static,
    close: impl Fn(GameState<T>, Sprites) + 'static,
) {
    let (state, sprites) = init();

    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new()
        .with_inner_size(glutin::dpi::LogicalSize::new(state.width, state.height))
        .with_title(&state.title);

    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).expect("genji failed to make a display");

    let shaders = graphics::shaders::Shaders::new(&display);

    let mut sprite_cache = helpers::sprite_filter(sprites.as_ref().clone());
    let mut last = Instant::now();
    let mut closed = false;

    let mut state = Some(state);
    let mut sprites = Some(sprites);
    event_loop.run(move |ev, _, control_flow| {
        if closed {
            // panic!("glutin didn't close");
            return;
        }
        let state_ref = state.as_mut().unwrap();
        match ev {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::Resized(size) => {
                    display.gl_window().resize(size);
                    state.as_mut().unwrap().width = size.width;
                    state.as_mut().unwrap().height = size.height;
                }
                glutin::event::WindowEvent::CloseRequested => {
                    if closed {
                        panic!("glutin tried to close twice");
                    }
                    if state.as_ref().unwrap().close_on_request {
                        control_flow.set_exit();
                        close(state.take().unwrap(), sprites.take().unwrap());
                        closed = true;
                    } else {
                        state.as_mut().unwrap().asked_to_close = true;
                    }
                }
                glutin::event::WindowEvent::ModifiersChanged(modifiers) => {
                    state_ref.keys[Key::Alt] = modifiers.alt();
                    state_ref.keys[Key::Ctrl] = modifiers.ctrl();
                    state_ref.keys[Key::Shift] = modifiers.shift();
                    state_ref.keys[Key::Super] = modifiers.logo();
                }
                glutin::event::WindowEvent::KeyboardInput { input, .. } => {
                    if let Some(ks) = Key::from_virtual(input.virtual_keycode) {
                        for key in ks {
                            match input.state {
                                glutin::event::ElementState::Pressed => state_ref.keys[key] = true,
                                glutin::event::ElementState::Released => {
                                    state_ref.keys[key] = false
                                }
                            }
                        }
                    } else if let Some(key) = Key::from_keycode(input.scancode) {
                        match input.state {
                            glutin::event::ElementState::Pressed => state_ref.keys[key] = true,
                            glutin::event::ElementState::Released => state_ref.keys[key] = false,
                        }
                    }
                }
                glutin::event::WindowEvent::MouseWheel { delta, .. } => {
                    state_ref.scroll = match delta {
                        glutin::event::MouseScrollDelta::LineDelta(x, y) => ((x + y) * 40.0).ceil() as i32,
                        glutin::event::MouseScrollDelta::PixelDelta(s) => gl2gj::pxcoord(s.x + s.y, state_ref.height),
                    };
                }
                glutin::event::WindowEvent::MouseInput { state, button, .. } => {
                    let key = match button {
                        glutin::event::MouseButton::Left => Key::LClick,
                        glutin::event::MouseButton::Right => Key::RClick,
                        glutin::event::MouseButton::Middle => Key::MClick,
                        glutin::event::MouseButton::Other(i) => Key::M1 + (i % 4),
                    };

                    match state {
                        glutin::event::ElementState::Pressed => state_ref.keys[key] = true,
                        glutin::event::ElementState::Released => state_ref.keys[key] = false,
                    }
                }

                glutin::event::WindowEvent::CursorMoved { position, .. } => {
                    let (x, y): (f64, f64) = position.into();
                    state_ref.mouse_x = gl2gj::pxcoord(x, state_ref.width);
                    state_ref.mouse_y = gl2gj::pxcoord(-y, state_ref.height);
                }

                _ => {}
            },

            glutin::event::Event::RedrawEventsCleared => {
                display.gl_window().window().request_redraw();
            }

            glutin::event::Event::RedrawRequested(_) => {
                let sprites_ref = sprites.as_mut().unwrap();
                if onloop(state_ref, sprites_ref) {
                    control_flow.set_exit();
                    if closed {
                        panic!("glutin tried to close twice");
                    }
                    close(state.take().unwrap(), sprites.take().unwrap());
                    closed = true;
                    return;
                }

                state_ref.delta = (Instant::now() - last).as_millis();
                if state_ref.delta < state_ref.fps {
                    thread::sleep(Duration::from_millis(
                        (state_ref.fps - state_ref.delta) as u64,
                    ));
                    state_ref.delta = state_ref.fps;
                }
                last = Instant::now();

                let mut target = display.draw();
                if unsafe { *SPRITES_CHANGED } {
                    sprite_cache = helpers::sprite_filter(sprites_ref.as_ref().clone());
                    unsafe { *SPRITES_CHANGED = false };
                }

                if let Some(col) = state_ref.clear_color {
                    let col = col.to_f32();
                    target.clear_color_and_depth((col[0], col[1], col[2], col[3]), 1.0);
                }

                for sprite in &sprite_cache {
                    sprite.draw(&mut target, &display, &shaders);
                }

                target.finish().expect("failed to swap buffers");

                state_ref.scroll = 0;
            }

            _ => {}
        }
    });
}
