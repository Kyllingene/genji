#![doc = include_str!("../lib-doc.md")]

// TODO: add audio

use std::thread;
use std::time::{Duration, Instant};

pub use genji_macros::init;

pub mod ecs;
pub mod graphics;
pub mod input;
pub mod prelude;
pub mod state;

use input::Key;

use ecs::World;
use glium::{glutin, Surface};
use graphics::{
    sprite::{Circle, Rect, Text, Texture, Triangle},
    Angle, Color, Depth, Fill, Position, Sprite, SpriteData, StrokeWeight,
};
use state::GameState;

mod helpers;
use helpers::gl2gj;

/// Runs the engine code for genji. Automatically run
/// via `genji::init`, so please don't do this manually.
#[doc(hidden)]
pub fn main<T: 'static>(
    init: fn() -> (GameState<T>, World),
    onloop: fn(&mut GameState<T>, &mut World) -> bool,
    close: fn(GameState<T>, World),
) {
    let (state, world) = init();

    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new()
        .with_inner_size(glutin::dpi::LogicalSize::new(state.width, state.height))
        .with_title(&state.title);

    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).expect("genji failed to make a display");

    let shaders = graphics::shaders::Shaders::new(&display);

    // let mut sprite_cache = helpers::sprite_filter(world.as_ref().clone());
    let mut last = Instant::now();
    let mut closed = false;

    let mut state = Some(state);
    let mut world = Some(world);
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
                        close(state.take().unwrap(), world.take().unwrap());
                        closed = true;
                    }

                    state.as_mut().unwrap().asked_to_close = true;
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
                        glutin::event::MouseScrollDelta::LineDelta(x, y) => {
                            ((x + y) * 40.0).ceil() as i32
                        }
                        glutin::event::MouseScrollDelta::PixelDelta(s) => {
                            gl2gj::pxcoord(s.x + s.y, state_ref.height)
                        }
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
                let world_ref = world.as_mut().unwrap();
                if onloop(state_ref, world_ref) {
                    control_flow.set_exit();
                    if closed {
                        panic!("glutin tried to close twice");
                    }
                    close(state.take().unwrap(), world.take().unwrap());
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
                // if unsafe { *SPRITES_CHANGED } {
                //     sprite_cache = helpers::sprite_filter(sprites_ref.as_ref().clone());
                //     unsafe { *SPRITES_CHANGED = false };
                // }

                if let Some(col) = state_ref.clear_color {
                    let col = col.to_f32();
                    target.clear_color_and_depth((col[0], col[1], col[2], col[3]), 1.0);
                }

                let mut sorted = Vec::new();
                macro_rules! draw_sprites {
                    ( $( $sprite_type:ident ),* ) => {$(
                        let mut query = world_ref.query::<(&$sprite_type, &Position)>();
                        for (id, (sprite, pos)) in query.iter() {
                            let mut ex = SpriteData::new();
                            ex.x = pos.0;
                            ex.y = pos.1;

                            if let Ok(angle) = world_ref.get::<&Angle>(id) {
                                ex.angle = **angle;
                            }

                            if let Ok(color) = world_ref.get::<&Color>(id) {
                                ex.color = *color;
                            }

                            if let Ok(depth) = world_ref.get::<&Depth>(id) {
                                ex.depth = **depth;
                            }

                            if let Ok(fill) = world_ref.get::<&Fill>(id) {
                                ex.fill = **fill;
                            }

                            if let Ok(stroke_weight) = world_ref.get::<&StrokeWeight>(id) {
                                ex.stroke_weight = **stroke_weight;
                            }

                            sorted.push((Sprite::$sprite_type(sprite), ex));
                        }
                    )*};
                }

                draw_sprites!(Rect, Circle, Triangle, Text, Texture);
                sorted.sort_by(|(_, ex1), (_, ex2)| ex2.depth.cmp(&ex1.depth));
                for (sprite, ex) in sorted.into_iter().filter(|(_, ex)| ex.depth > 0) {
                    sprite.draw(&mut target, ex, &display, &shaders);
                }

                target.finish().expect("failed to swap buffers");

                state_ref.scroll = 0;
            }

            _ => {}
        }
    });
}
