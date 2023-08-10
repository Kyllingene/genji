use std::thread;
use std::time::{Duration, Instant};

pub use genji_macros::init;

pub mod graphics;
pub mod input;
pub mod state;

use input::Key;

use glium::{glutin, Surface};

pub mod prelude;

mod helpers;

/// Runs the engine code for genji. Automatically run
/// via `genji::init`, so please don't do this manually.
pub fn main<T: 'static>(
    init: impl FnOnce() -> state::GameState<T> + 'static,
    onloop: impl Fn(&mut state::GameState<T>) -> bool + 'static,
    close: impl Fn(state::GameState<T>) + 'static,
) {
    let state = init();

    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new()
        .with_inner_size(glutin::dpi::LogicalSize::new(state.width, state.height))
        .with_title(&state.title);

    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).expect("genji failed to make a display");

    let shaders = graphics::shaders::Shaders::new(&display);

    let mut sprites = helpers::sprite_filter(state.sprites.clone());
    let mut last = Instant::now();
    let mut closed = false;

    let mut state = Some(state);
    event_loop.run(move |ev, _, control_flow| match ev {
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
                    close(state.take().unwrap());
                    closed = true;
                } else {
                    state.as_mut().unwrap().asked_to_close = true;
                }
            }
            glutin::event::WindowEvent::KeyboardInput { input, .. } => {
                if let Some(ks) = Key::from_virtual(input.virtual_keycode) {
                    for key in ks {
                        match input.state {
                            glutin::event::ElementState::Pressed => state.as_mut().unwrap().keys[key] = true,
                            glutin::event::ElementState::Released => state.as_mut().unwrap().keys[key] = false,
                        }
                    }
                } else if let Some(key) = Key::from_keycode(input.scancode) {
                    match input.state {
                        glutin::event::ElementState::Pressed => state.as_mut().unwrap().keys[key] = true,
                        glutin::event::ElementState::Released => state.as_mut().unwrap().keys[key] = false,
                    }
                }
            }

            _ => {}
        },

        glutin::event::Event::RedrawEventsCleared => {
            display.gl_window().window().request_redraw();
        }

        glutin::event::Event::RedrawRequested(_) => {
            if closed {
                // panic!("glutin didn't close");
                return;
            }

            let state_ref = state.as_mut().unwrap();
            if onloop(state_ref) {
                control_flow.set_exit();
                if closed {
                    panic!("glutin tried to close twice");
                }
                close(state.take().unwrap());
                closed = true;
                return;
            }

            state_ref.delta = (Instant::now() - last).as_millis();
            if state_ref.delta < state_ref.fps {
                thread::sleep(Duration::from_millis((state_ref.fps - state_ref.delta) as u64));
                state_ref.delta = state_ref.fps;
            }
            last = Instant::now();

            let mut target = display.draw();
            if state_ref.sprites_changed {
                sprites = helpers::sprite_filter(state_ref.sprites.clone());
                state_ref.sprites_changed = false;
            }

            if let Some(col) = state_ref.clear_color {
                let col = col.to_f32();
                target.clear_color_and_depth((col[0], col[1], col[2], col[3]), 1.0);
            }

            for sprite in &sprites {
                sprite.draw(&mut target, &display, &shaders);
            }

            target.finish().expect("failed to swap buffers");
        }

        _ => {}
    });
}
