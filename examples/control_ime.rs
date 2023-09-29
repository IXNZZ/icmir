use std::{env, path};
use ggez::conf::{WindowMode, WindowSetup};
use ggez::{Context, event, GameError, GameResult};
use ggez::audio::AudioContext;
use ggez::event::{EventHandler, EventLoop};
use ggez::input::keyboard::{KeyboardContext, KeyCode, KeyInput};
use ggez::winit::dpi::{PhysicalPosition, Position};
use ggez::winit::window::ImePurpose;
use tracing::{debug, info, warn};
use tracing_subscriber::EnvFilter;
use tracing_subscriber::fmt::format::Writer;
use tracing_subscriber::fmt::time::FormatTime;

struct LocalTimer;

impl FormatTime for LocalTimer {
    fn format_time(&self, w: &mut Writer<'_>) -> std::fmt::Result {
        write!(w, "{}", chrono::Local::now().format("%F %T%.6f"))
    }
}

fn main() -> GameResult {

    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "warn,window=debug")
    }
    tracing_subscriber::fmt::fmt()
        .with_timer(LocalTimer)
        .with_thread_names(true)
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    // We add the CARGO_MANIFEST_DIR/resources to the resource paths
    // so that ggez will look in our cargo project directory for files.
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };

    let cb = ggez::ContextBuilder::new("map-distribution", "icmir2")
        .add_resource_path(resource_dir)
        .window_setup(WindowSetup::default().title("map-distribution"))
        .window_mode(WindowMode::default().dimensions(1400.0, 1200.0));

    let (mut ctx, event_loop) = cb.build()?;



    let mut app = App::new(&mut ctx);
    // state.load_tile(&mut ctx, 0);
    my_event::run(ctx, event_loop, app)
}

pub struct App {

}

impl App {

    pub fn new(ctx: &mut Context) -> Self {
        let size = ctx.gfx.size();
        debug!("new: {:?}", size);
        if let Some(monitor) = ctx.gfx.window().current_monitor() {
            let scale_factor = monitor.scale_factor();

            debug!("monitor: {:?}", monitor.size());
        }
        App {}
    }
}

impl EventHandler for App {
    fn update(&mut self, _ctx: &mut Context) -> Result<(), GameError> {
        Ok(())
    }

    fn draw(&mut self, _ctx: &mut Context) -> Result<(), GameError> {
        Ok(())
    }

    fn key_down_event(&mut self, ctx: &mut Context, input: KeyInput, _repeated: bool) -> Result<(), GameError> {
        debug!("key_down_event: {:?}", input);
        Ok(())
    }

    fn key_up_event(&mut self, ctx: &mut Context, _input: KeyInput) -> Result<(), GameError> {
        debug!("key_up_event: {:?}", _input);
        if let Some(x) = _input.keycode {
            if x == KeyCode::Return {
                info!("key_up_event: {:?}", _input);
                ctx.gfx.window().set_ime_allowed(true);
                ctx.gfx.window().set_ime_purpose(ImePurpose::Normal);
                ctx.gfx.window().set_ime_position(Position::Physical(PhysicalPosition {x: 20, y: 20}));
                if let Some(monitor) = ctx.gfx.window().current_monitor() {
                    info!("monitor: {:?}", monitor);
                }
            }
            if x == KeyCode::Escape {
                ctx.gfx.window().set_ime_allowed(false);

            }
        }
        Ok(())
    }

    fn text_input_event(&mut self, _ctx: &mut Context, _character: char) -> Result<(), GameError> {
        debug!("text_input_event: {:?}", _character);
        Ok(())
    }
}

mod my_event {
    use ggez::{Context, event};
    use ggez::event::{ErrorOrigin, EventHandler};
    use ggez::input::keyboard::{KeyInput, KeyMods};
    use tracing::error;
    use winit::dpi;
    use winit::event::{ElementState, Event, Ime, KeyboardInput, MouseScrollDelta, WindowEvent};
    use winit::event_loop::{ControlFlow, EventLoop};

    pub fn run<S: 'static, E>(mut ctx: Context, event_loop: EventLoop<()>, mut state: S) -> !
        where
            S: EventHandler<E>,
            E: std::fmt::Debug,
    {
        event_loop.run(move |mut event, _, control_flow| {
            let ctx = &mut ctx;
            let state = &mut state;

            if ctx.quit_requested {
                let res = state.quit_event(ctx);
                ctx.quit_requested = false;
                if let Ok(false) = res {
                    ctx.continuing = false;
                } else if catch_error(ctx, res, state, control_flow, ErrorOrigin::QuitEvent) {
                    return;
                }
            }
            if !ctx.continuing {
                *control_flow = ControlFlow::Exit;
                return;
            }

            *control_flow = ControlFlow::Poll;

            event::process_event(ctx, &mut event);
            match event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::Resized(logical_size) => {
                        // let actual_size = logical_size;
                        let res = state.resize_event(
                            ctx,
                            logical_size.width as f32,
                            logical_size.height as f32,
                        );
                        if catch_error(ctx, res, state, control_flow, ErrorOrigin::ResizeEvent) {
                            return;
                        };
                    }
                    WindowEvent::CloseRequested => {
                        let res = state.quit_event(ctx);
                        if let Ok(false) = res {
                            ctx.continuing = false;
                        } else if catch_error(ctx, res, state, control_flow, ErrorOrigin::QuitEvent) {
                            return;
                        }
                    }
                    WindowEvent::Focused(gained) => {
                        let res = state.focus_event(ctx, gained);
                        if catch_error(ctx, res, state, control_flow, ErrorOrigin::FocusEvent) {
                            return;
                        };
                    }
                    WindowEvent::ReceivedCharacter(ch) => {
                        let res = state.text_input_event(ctx, ch);
                        if catch_error(ctx, res, state, control_flow, ErrorOrigin::TextInputEvent) {
                            return;
                        };
                    }
                    WindowEvent::ModifiersChanged(mods) => {
                        ctx.keyboard.set_modifiers(KeyMods::from(mods))
                    }
                    WindowEvent::KeyboardInput {
                        input:
                        KeyboardInput {
                            state: ElementState::Pressed,
                            virtual_keycode: keycode,
                            scancode,
                            ..
                        },
                        ..
                    } => {
                        let repeat = ctx.keyboard.is_key_repeated();
                        let res = state.key_down_event(
                            ctx,
                            KeyInput {
                                scancode,
                                keycode,
                                mods: ctx.keyboard.active_mods(),
                            },
                            repeat,
                        );
                        if catch_error(ctx, res, state, control_flow, ErrorOrigin::KeyDownEvent) {
                            return;
                        };
                    }
                    WindowEvent::KeyboardInput {
                        input:
                        KeyboardInput {
                            state: ElementState::Released,
                            virtual_keycode: keycode,
                            scancode,
                            ..
                        },
                        ..
                    } => {
                        let res = state.key_up_event(
                            ctx,
                            KeyInput {
                                scancode,
                                keycode,
                                mods: ctx.keyboard.active_mods(),
                            },
                        );
                        if catch_error(ctx, res, state, control_flow, ErrorOrigin::KeyUpEvent) {
                            return;
                        };
                    }
                    WindowEvent::MouseWheel { delta, .. } => {
                        let (x, y) = match delta {
                            MouseScrollDelta::LineDelta(x, y) => (x, y),
                            MouseScrollDelta::PixelDelta(pos) => {
                                let scale_factor = ctx.gfx.window().scale_factor();
                                let dpi::LogicalPosition { x, y } = pos.to_logical::<f32>(scale_factor);
                                (x, y)
                            }
                        };
                        let res = state.mouse_wheel_event(ctx, x, y);
                        if catch_error(ctx, res, state, control_flow, ErrorOrigin::MouseWheelEvent) {
                            return;
                        };
                    }
                    WindowEvent::MouseInput {
                        state: element_state,
                        button,
                        ..
                    } => {
                        let position = ctx.mouse.position();
                        match element_state {
                            ElementState::Pressed => {
                                let res =
                                    state.mouse_button_down_event(ctx, button, position.x, position.y);
                                if catch_error(
                                    ctx,
                                    res,
                                    state,
                                    control_flow,
                                    ErrorOrigin::MouseButtonDownEvent,
                                ) {
                                    return;
                                };
                            }
                            ElementState::Released => {
                                let res =
                                    state.mouse_button_up_event(ctx, button, position.x, position.y);
                                if catch_error(
                                    ctx,
                                    res,
                                    state,
                                    control_flow,
                                    ErrorOrigin::MouseButtonUpEvent,
                                ) {
                                    return;
                                };
                            }
                        }
                    }
                    WindowEvent::CursorMoved { .. } => {
                        let position = ctx.mouse.position();
                        let delta = ctx.mouse.last_delta();
                        let res =
                            state.mouse_motion_event(ctx, position.x, position.y, delta.x, delta.y);
                        if catch_error(ctx, res, state, control_flow, ErrorOrigin::MouseMotionEvent) {
                            return;
                        };
                    }
                    WindowEvent::Touch(touch) => {
                        let res =
                            state.touch_event(ctx, touch.phase, touch.location.x, touch.location.y);
                        if catch_error(ctx, res, state, control_flow, ErrorOrigin::TouchEvent) {
                            return;
                        };
                    }
                    WindowEvent::CursorEntered { device_id: _ } => {
                        let res = state.mouse_enter_or_leave(ctx, true);
                        if catch_error(
                            ctx,
                            res,
                            state,
                            control_flow,
                            ErrorOrigin::MouseEnterOrLeave,
                        ) {
                            return;
                        }
                    }
                    WindowEvent::CursorLeft { device_id: _ } => {
                        let res = state.mouse_enter_or_leave(ctx, false);
                        if catch_error(
                            ctx,
                            res,
                            state,
                            control_flow,
                            ErrorOrigin::MouseEnterOrLeave,
                        ) {
                            return;
                        }
                    },
                    WindowEvent::Ime(ime) => {
                        match ime {
                            Ime::Enabled => {
                                println!("Ime Enabled");
                            }
                            Ime::Preedit(str, size) => {
                                println!("Ime Preedit: {str}, {:?}", size);
                            }
                            Ime::Commit(str) => {
                                println!("Ime Commit: {str}");
                            }
                            Ime::Disabled => {
                                println!("Ime Disabled");
                            }
                        }
                    },
                    _x => {
                        // trace!("ignoring window event {:?}", x);
                    }
                },
                Event::DeviceEvent { .. } => (),
                Event::Resumed => (),
                Event::Suspended => (),
                Event::NewEvents(_) => (),
                Event::UserEvent(_) => (),
                Event::MainEventsCleared => {
                    // If you are writing your own event loop, make sure
                    // you include `timer_context.tick()` and
                    // `ctx.process_event()` calls.  These update ggez's
                    // internal state however necessary.
                    ctx.time.tick();

                    // Handle gamepad events if necessary.
                    #[cfg(feature = "gamepad")]
                    while let Some(gilrs::Event { id, event, .. }) = ctx.gamepad.next_event() {
                        match event {
                            gilrs::EventType::ButtonPressed(button, _) => {
                                let res = state.gamepad_button_down_event(ctx, button, GamepadId(id));
                                if catch_error(
                                    ctx,
                                    res,
                                    state,
                                    control_flow,
                                    ErrorOrigin::GamepadButtonDownEvent,
                                ) {
                                    return;
                                };
                            }
                            gilrs::EventType::ButtonReleased(button, _) => {
                                let res = state.gamepad_button_up_event(ctx, button, GamepadId(id));
                                if catch_error(
                                    ctx,
                                    res,
                                    state,
                                    control_flow,
                                    ErrorOrigin::GamepadButtonUpEvent,
                                ) {
                                    return;
                                };
                            }
                            gilrs::EventType::AxisChanged(axis, value, _) => {
                                let res = state.gamepad_axis_event(ctx, axis, value, GamepadId(id));
                                if catch_error(
                                    ctx,
                                    res,
                                    state,
                                    control_flow,
                                    ErrorOrigin::GamepadAxisEvent,
                                ) {
                                    return;
                                };
                            }
                            _ => {}
                        }
                    }

                    let res = state.update(ctx);
                    if catch_error(ctx, res, state, control_flow, ErrorOrigin::Update) {
                        return;
                    };

                    if let Err(e) = ctx.gfx.begin_frame() {
                        error!("Error on GraphicsContext::begin_frame(): {e:?}");
                        eprintln!("Error on GraphicsContext::begin_frame(): {e:?}");
                        *control_flow = ControlFlow::Exit;
                    }

                    if let Err(e) = state.draw(ctx) {
                        error!("Error on EventHandler::draw(): {e:?}");
                        eprintln!("Error on EventHandler::draw(): {e:?}");
                        if state.on_error(ctx, ErrorOrigin::Draw, e) {
                            *control_flow = ControlFlow::Exit;
                            return;
                        }
                    }

                    if let Err(e) = ctx.gfx.end_frame() {
                        error!("Error on GraphicsContext::end_frame(): {e:?}");
                        eprintln!("Error on GraphicsContext::end_frame(): {e:?}");
                        *control_flow = ControlFlow::Exit;
                    }

                    // reset the mouse delta for the next frame
                    // necessary because it's calculated cumulatively each cycle
                    ctx.mouse.reset_delta();

                    // Copy the state of the keyboard into the KeyboardContext
                    // and the mouse into the MouseContext
                    ctx.keyboard.save_keyboard_state();
                    ctx.mouse.save_mouse_state();
                }
                Event::RedrawRequested(_) => (),
                Event::RedrawEventsCleared => (),
                Event::LoopDestroyed => (),
            }
        })
    }

    fn catch_error<T, E, S: 'static>(
        ctx: &mut Context,
        event_result: Result<T, E>,
        state: &mut S,
        control_flow: &mut ControlFlow,
        origin: ErrorOrigin,
    ) -> bool
        where
            S: EventHandler<E>,
            E: std::fmt::Debug,
    {
        if let Err(e) = event_result {
            error!("Error on EventHandler {origin:?}: {e:?}");
            eprintln!("Error on EventHandler {origin:?}: {e:?}");
            if state.on_error(ctx, origin, e) {
                *control_flow = ControlFlow::Exit;
                return true;
            }
        }
        false
    }
}