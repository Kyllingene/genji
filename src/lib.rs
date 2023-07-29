use std::collections::HashMap;
use std::sync::mpsc::channel;
use std::thread;
use std::time::{Duration, Instant};

pub use genji_macros::init;

pub mod graphics;
pub mod input;
pub mod state;
use glium::{glutin, Surface};
use graphics::{Color, Sprite};

// pub mod prelude;

mod helpers;

// const DELTA_MIN: u128 = 20; // 20 = 30fps, 17 = 60fps

enum Message {
    Resize(u32, u32),
}

/// Runs the engine code for genji. Automatically run
/// via `genji_init`.
pub fn main<T: Send>(
    init: impl FnOnce() -> state::GameState<T> + std::marker::Send + 'static,
    onloop: impl Fn(&mut state::GameState<T>) -> bool + std::marker::Send + 'static,
    close: impl FnOnce(state::GameState<T>) + std::marker::Send + 'static,
) {
    let mut state = init();
    let mut clear_color = state.clear_color;
    let mut sprites = state.sprites.clone();

    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new()
        .with_inner_size(glutin::dpi::LogicalSize::new(state.width, state.height))
        .with_title(&state.title);

    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).expect("genji failed to make a display");

    let shaders = graphics::shaders::Shaders::new(&display);

    let (tx_sprites, rx_sprites) = channel::<Option<(Option<Color>, HashMap<u64, Sprite>)>>();
    let (tx_msg, rx_msg) = channel();
    thread::scope(|s| {
        s.spawn(move || {
            let mut exit = false;
            let mut last = Instant::now();
            while !onloop(&mut state) {
                state.delta = (Instant::now() - last).as_millis();
                if state.delta < state.fps {
                    thread::sleep(Duration::from_millis((state.fps - state.delta) as u64));
                    state.delta = state.fps;
                }
                last = Instant::now();

                if let Ok(msg) = rx_msg.try_recv() {
                    match msg {
                        Message::Resize(w, h) => {
                            state.width = w;
                            state.height = h;
                        }
                    }
                }

                tx_sprites
                    .send(Some((state.clear_color, state.sprites.clone())))
                    .unwrap_or_else(|_| exit = true);

                if exit {
                    break;
                }
            }

            _ = tx_sprites.send(None);

            close(state);
        });

        event_loop.run(move |ev, _, control_flow| match ev {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::Resized(size) => {
                    display.gl_window().resize(size);
                    tx_msg
                        .send(Message::Resize(size.width, size.height))
                        .unwrap_or_else(|_| control_flow.set_exit());
                }
                glutin::event::WindowEvent::CloseRequested => {
                    control_flow.set_exit();
                    return;
                }

                _ => {}
            },

            glutin::event::Event::RedrawEventsCleared => {
                display.gl_window().window().request_redraw()
            }

            glutin::event::Event::RedrawRequested(_) => {
                if let Ok(Some((cc, ss))) = rx_sprites.try_recv() {
                    clear_color = cc;
                    sprites = ss;
                }

                let mut target = display.draw();

                if let Some(col) = clear_color {
                    let col = col.to_f32();
                    target.clear_color_and_depth((col[0], col[1], col[2], col[3]), 1.0);
                }

                for (_id, sprite) in &sprites {
                    sprite.draw(&mut target, &display, &shaders);
                }

                target.finish().expect("failed to swap buffers");
            }

            _ => {}
        });
    });
}
