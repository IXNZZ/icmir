use std::{env, path};
use file::asset::{FileDesc, FileDescType, ImageAsset};
use ggez::conf::{WindowMode, WindowSetup};
use ggez::{Context, event, GameError, GameResult};
use ggez::audio::AudioContext;
use ggez::event::{EventHandler, EventLoop};
use ggez::glam::vec2;
use ggez::graphics::{BlendComponent, BlendFactor, BlendMode, BlendOperation, Canvas, Color, DrawMode, DrawParam, FillOptions, ImageEncodingFormat, ImageFormat, Mesh, Rect};
use ggez::input::keyboard::{KeyboardContext, KeyCode, KeyInput};
use ggez::mint::Point2;
use ggez::winit::dpi::{PhysicalPosition, Position};
use ggez::winit::window::ImePurpose;
use image::RgbaImage;
use keyframe::{AnimationSequence, keyframes};
use keyframe_derive::CanTween;
use tracing::{debug, info, warn};
use tracing_subscriber::EnvFilter;
use tracing_subscriber::fmt::format::Writer;
use tracing_subscriber::fmt::time::FormatTime;
use winit::event::{MouseButton, VirtualKeyCode};
use crate::animation::{Direction, PlayerAnimation, PlayerAction};

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

    let cb = ggez::ContextBuilder::new("hum-distribution", "icmir2")
        .add_resource_path(resource_dir)
        .window_setup(WindowSetup::default().title("hum-distribution"))
        .window_mode(WindowMode::default().dimensions(1400.0, 1200.0));

    let (mut ctx, event_loop) = cb.build()?;



    let mut app = App::new(&mut ctx);
    // state.load_tile(&mut ctx, 0);
    my_event::run(ctx, event_loop, app)
}





pub struct App {
    asset: ImageAsset,
    scale_factor: f32,
    animation: animation::PlayerAnimation,
}

impl App {

    pub fn new(ctx: &mut Context) -> Self {
        let size = ctx.gfx.size();
        debug!("new: {:?}", size);
        let mut scale_factor = 1.0;
        if let Some(monitor) = ctx.gfx.window().current_monitor() {
            scale_factor = monitor.scale_factor();
            debug!("monitor: {:?}, scale: {}", monitor.size(), scale_factor);
        }
        let mut asset = file::asset::create_default_image_asset("/Users/vinter/Dev/Mir2");
        asset.put_file_map(4, "hum", true);
        asset.put_file_map(5, "hair", true);
        asset.put_file_map(6, "weapon", true);
        asset.put_file_map(7, "humeffect", true);

        let data = asset.load_image(FileDesc::KEY(9007216434610196), FileDescType::IDX).unwrap();
        //9007216434610196
        // let image = RgbaImage::from_raw(data.width as u32, data.height as u32, data.bytes.to_vec()).unwrap();
        let image = ggez::graphics::Image::from_pixels(ctx, data.bytes.as_ref(), ImageFormat::Rgba8UnormSrgb, data.width as u32, data.height as u32);

        let animation = PlayerAnimation::new(4, 0, 1, PlayerAction::Stand, Direction::North);
        App { asset, scale_factor: scale_factor as f32 + 0.5, animation}
    }
}

impl EventHandler for App {
    fn update(&mut self, ctx: &mut Context) -> Result<(), GameError> {
        let time = ctx.time.delta().as_secs_f64();
        // println!("time: {}", time);
        self.animation.advance(time);
        // println!("advance: {}, time: {time}", self.player_animation.duration());
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> Result<(), GameError> {
        let mut canvas = Canvas::from_frame(ctx, Color::new(0.1, 0.2, 0.3, 1.0));
        // let frame = self.animation.now();
        // println!("frame: {}", frame);
        // for i in 0..4 {
        let hum_image = self.asset.load_image(FileDesc::ZONE { file: self.animation.file, number: self.animation.number, index: self.animation.now() as u32 }, FileDescType::IDX);
        if let Some(hum) = hum_image {
            // println!("H:{},W:{},Len: {}", img.height, img.width, img.bytes.len());
            let image = ggez::graphics::Image::from_pixels(ctx, hum.bytes.as_ref(), ImageFormat::Rgba8UnormSrgb, hum.width as u32, hum.height as u32);
            canvas.draw(&image, DrawParam::new()
                // .offset(vec2(img.offset_x as f32, img.offset_y as f32))
                .scale(vec2(self.scale_factor, self.scale_factor))
                // .dest(vec2(500.0, 500.0)));
                .dest(vec2(  500.0 + hum.offset_x as f32 * self.scale_factor, 500.0 + hum.offset_y as f32 * self.scale_factor)));
            // let image = ggez::graphics::Image::from_pixels(ctx, hair_image.bytes.as_ref(), ImageFormat::Rgba8UnormSrgb, hair_image.width as u32, hair_image.height as u32);
            // canvas.draw(&image, DrawParam::new()
            //     // .offset(vec2(img.offset_x as f32, img.offset_y as f32))
            //     .scale(vec2(self.scale_factor, self.scale_factor))
            //     // .dest(vec2(500.0, 500.0)));
            //     .dest(vec2(  500.0 + hair_image.offset_x as f32 * self.scale_factor, 500.0 + hair_image.offset_y as f32 * self.scale_factor )));
        }
        let hair_image = self.asset.load_image(FileDesc::ZONE { file: 5, number: 0, index: self.animation.hair() as u32 + 1248 }, FileDescType::IDX);
        if let Some(hair) = hair_image {
            // println!("H:{},W:{},Len: {}", img.height, img.width, img.bytes.len());
            let image = ggez::graphics::Image::from_pixels(ctx, hair.bytes.as_ref(), ImageFormat::Rgba8UnormSrgb, hair.width as u32, hair.height as u32);
            // image.encode()
            canvas.draw(&image, DrawParam::new()
                // .offset(vec2(img.offset_x as f32, img.offset_y as f32))
                .scale(vec2(self.scale_factor, self.scale_factor))
                // .dest(vec2(500.0, 500.0)));
                .dest(vec2(  500.0 + hair.offset_x as f32 * self.scale_factor, 500.0 + hair.offset_y as f32 * self.scale_factor )))

        }

        // canvas.set_blend_mode(BlendMode {
        //     color: BlendComponent {
        //         src_factor: BlendFactor::One,
        //         dst_factor: BlendFactor::OneMinusSrcAlpha,
        //         operation: BlendOperation::Add
        //     },
        //     alpha: BlendComponent {
        //         src_factor: BlendFactor::Src,
        //         dst_factor: BlendFactor::Zero,
        //         operation: BlendOperation::Add
        //     }
        // });
        canvas.set_blend_mode(BlendMode::ADD);

        let weapon_image = self.asset.load_image(FileDesc::ZONE { file: 6, number: 0, index: self.animation.hair() as u32 + 2496 + 416 }, FileDescType::IDX);
        if let Some(weapon) = weapon_image {
            // println!("H:{},W:{},Len: {}", img.height, img.width, img.bytes.len());
            let image = ggez::graphics::Image::from_pixels(ctx, weapon.bytes.as_ref(), ImageFormat::Rgba8UnormSrgb, weapon.width as u32, weapon.height as u32);
            canvas.draw(&image, DrawParam::new()
                // .offset(vec2(img.offset_x as f32, img.offset_y as f32))
                .scale(vec2(self.scale_factor, self.scale_factor))
                // .dest(vec2(500.0, 500.0)));
                .dest(vec2(  500.0 + weapon.offset_x as f32 * self.scale_factor, 500.0 + weapon.offset_y as f32 * self.scale_factor )))

        }


        let effect_image = self.asset.load_image(FileDesc::ZONE { file: 7, number: 0, index: self.animation.effect() as u32 + 448 }, FileDescType::IDX);
        if let Some(effect) = effect_image {
            // println!("H:{},W:{},Len: {}", img.height, img.width, img.bytes.len());
            let image = ggez::graphics::Image::from_pixels(ctx, effect.bytes.as_ref(), ImageFormat::Rgba8UnormSrgb, effect.width as u32, effect.height as u32);
            // let (wgpu, view) = image.wgpu();

            canvas.draw(&image, DrawParam::new()
                // .offset(vec2(img.offset_x as f32, img.offset_y as f32))
                .scale(vec2(self.scale_factor, self.scale_factor))
                // .dest(vec2(500.0, 500.0)));
                .dest(vec2(  500.0 + effect.offset_x as f32 * self.scale_factor, 500.0 + effect.offset_y as f32 * self.scale_factor )))

        }

        canvas.set_blend_mode(BlendMode {
            color: BlendComponent {
                src_factor: BlendFactor::DstAlpha,
                dst_factor: BlendFactor::One,
                operation: BlendOperation::Add
            },
            alpha: BlendComponent {
                src_factor: BlendFactor::One,
                dst_factor: BlendFactor::One,
                operation: BlendOperation::Add
            }
        });
        let red_color = Mesh::new_rectangle(ctx,
                                            DrawMode::Fill(FillOptions::default()),
                                            Rect::new(0.0, 0.0, 500.0, 500.0),
                                            Color::new(1.0, 0.0, 0.0, 1.0))?;
        canvas.draw(&red_color,
                    DrawParam::new().dest(vec2(300.0, 300.0)));

        canvas.finish(ctx).unwrap();
        Ok(())
    }

    fn key_down_event(&mut self, ctx: &mut Context, input: KeyInput, _repeated: bool) -> Result<(), GameError> {
        // debug!("key_down_event: {:?}", input);
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
            match x {
                VirtualKeyCode::Key1 => {
                    // let player_animation: AnimationSequence<PlayerFrame> = keyframes![
                    //     (PlayerFrame::new(0, 0, 0, 0, 1.0), 0.0),
                    //     (PlayerFrame::new(0, 0, 0, 0, 4.9), 1.0)];
                    // let view = PlayerStateView::Stand { dir: Direction::South, sequence: player_animation, skip: 4.0, offset: 0.0 };
                    self.animation.state(PlayerAction::Stand);
                },
                VirtualKeyCode::Key2=> {
                    // let player_animation: AnimationSequence<PlayerFrame> = keyframes![
                    //     (PlayerFrame::new(0, 0, 0, 0, 1.0), 0.0),
                    //     (PlayerFrame::new(0, 0, 0, 0, 6.9), 1.0)];
                    // // println!("player frame: {}", player_animation.);
                    // let view = PlayerStateView::Stand { dir: Direction::South, sequence: player_animation, skip: 6.0, offset: 32.0 };
                    // self.player_view = view;
                    self.animation.state(PlayerAction::Walk);
                },
                VirtualKeyCode::Key3=> {
                    self.animation.state(PlayerAction::Run);
                },
                VirtualKeyCode::Key4=> {
                    self.animation.state(PlayerAction::WarMode);
                },
                VirtualKeyCode::Key5=> {
                    self.animation.state(PlayerAction::Hit);
                },
                VirtualKeyCode::Key6=> {
                    self.animation.state(PlayerAction::HeavyHit);
                },
                VirtualKeyCode::Key7=> {
                    self.animation.state(PlayerAction::BigHit);
                },
                VirtualKeyCode::Key8=> {
                    self.animation.state(PlayerAction::Spell);
                },
                VirtualKeyCode::Key9=> {
                    self.animation.state(PlayerAction::SitDown);
                },
                VirtualKeyCode::Key0=> {
                    self.animation.state(PlayerAction::Damage);
                },
                VirtualKeyCode::Minus=> {
                    self.animation.state(PlayerAction::Die);
                },
                // VirtualKeyCode::Equals=> {
                //     self.animation.state(PlayerAction::Die);
                // },


                VirtualKeyCode::A => {self.animation.dir(Direction::West)}
                VirtualKeyCode::D => {
                    self.animation.dir(Direction::East);
                    // let data = self.asset.load_image(FileDesc::KEY(9007216434610196), FileDescType::IDX).unwrap();
                    // //9007216434610196
                    // let image = RgbaImage::from_raw(data.width as u32, data.height as u32, data.bytes.to_vec()).unwrap();
                    // // let image = ggez::graphics::Image::from_pixels(ctx, data.bytes.as_ref(), ImageFormat::Rgba8UnormSrgb, data.width as u32, data.height as u32);
                    // image.save("/Users/vinter/Dev/Mir2/hum.png");
                    // image.encode(ctx, ImageEncodingFormat::Png, "/Users/vinter/Dev/Mir2/effect.png");
                }
                VirtualKeyCode::E => {self.animation.dir(Direction::Northeast)}
                VirtualKeyCode::Q => {self.animation.dir( Direction::Northwest)}
                VirtualKeyCode::R => {
                    self.animation.offset += 416;
                }
                VirtualKeyCode::T => {
                    // self.animation.offset -= 416;
                    if self.animation.offset > 416 {
                        self.animation.offset -= 416;
                    }
                }
                VirtualKeyCode::W => {self.animation.dir(Direction::North)}
                VirtualKeyCode::X => {self.animation.dir(Direction::South)}
                VirtualKeyCode::C => {self.animation.dir(Direction::Southeast)}
                VirtualKeyCode::Z => {self.animation.dir(Direction::Southwest)}
                KeyCode::F => {
                    self.animation.number += 1;
                    self.animation.offset = 1;
                    if self.animation.number == 1 {
                        self.animation.number += 1;
                    }
                }
                KeyCode::G => {
                    if self.animation.number == 0 {
                        return Ok(());
                    }
                    self.animation.number -= 1;
                    self.animation.offset = 1;
                    if self.animation.number == 1 {
                        self.animation.number -= 1;
                    }

                }
                _ => {}
            }
        }
        Ok(())
    }

    fn text_input_event(&mut self, _ctx: &mut Context, _character: char) -> Result<(), GameError> {
        // debug!("text_input_event: {:?}", _character);
        Ok(())
    }

    fn mouse_button_down_event(&mut self, _ctx: &mut Context, button: MouseButton, x: f32, y: f32) -> Result<(), GameError> {
        let angle = math::angle8(548.0, 532.0, x, y);
        debug!("x: {x}, y: {y}, button: {:?}, angle: {angle}", button);
        match angle {
            1.0 => {self.animation.dir(Direction::North)}
            2.0 => {self.animation.dir(Direction::Northeast)}
            3.0 => {self.animation.dir(Direction::East)}
            4.0 => {self.animation.dir(Direction::Southeast)}
            5.0 => {self.animation.dir(Direction::South)}
            6.0 => {self.animation.dir(Direction::Southwest)}
            7.0 => {self.animation.dir(Direction::West)}
            8.0 => {self.animation.dir(Direction::Northwest)}
            _ => {}
        }
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


mod animation {
    use keyframe::{AnimationSequence, keyframes};
    use keyframe::functions::Linear;
    use keyframe_derive::CanTween;

    #[derive(CanTween, Clone, Copy, Default)]
    pub struct PlayerFrame {
        // pub w: u32,
        // pub h: u32,
        // pub x: i32,
        // pub y: i32,
        pub count: f32,
    }

    impl PlayerFrame {
        pub fn new(count: f32) -> Self {
            Self {count}
        }
    }

    pub enum Direction {
        North, //北
        Northeast, //东北
        East, //东
        Southeast, // 东南
        South, // 南
        Southwest, // 西南
        West, //西
        Northwest // 西北
    }

    impl Direction {
        pub fn offset(&self) -> f32 {
            match self {
                Direction::North => {0.0}
                Direction::Northeast => {1.0}
                Direction::East => {2.0}
                Direction::Southeast => {3.0}
                Direction::South => {4.0}
                Direction::Southwest => {5.0}
                Direction::West => {6.0}
                Direction::Northwest => {7.0}
            }
        }
    }

    pub enum PlayerAction {
        Stand, //站立
        Walk, //步行
        Run, //跑步 { dir: Direction, sequence: AnimationSequence<PlayerFrame>, skip: f32, offset: f32},
        WarMode, //准备攻击
        Hit, //攻击
        HeavyHit, //重要的攻击
        BigHit, // 主要的攻击
        // FireHitReady, //魔法
        Spell, //魔法
        SitDown,
        Damage, //受到伤害
        Die,
    }

    impl PlayerAction {

        pub fn new_frame(&self) -> AnimationSequence<PlayerFrame> {
            match self {
                PlayerAction::Stand => {
                    keyframes![
                        (PlayerFrame::new(0.0), 0.0, Linear),
                        (PlayerFrame::new(4.0 - 0.001), 1.5, Linear)]
                }
                PlayerAction::Walk => {
                    keyframes![
                        (PlayerFrame::new(0.0), 0.0, Linear),
                        (PlayerFrame::new(6.0 - 0.001), 0.8, Linear)]
                }
                PlayerAction::Run => {
                    keyframes![
                        (PlayerFrame::new(0.0), 0.0, Linear),
                        (PlayerFrame::new(6.0 - 0.001), 1.0, Linear)]
                }
                PlayerAction::WarMode => {
                    keyframes![
                        (PlayerFrame::new(0.0), 0.0, Linear),
                        (PlayerFrame::new(1.0 - 0.001), 1.5, Linear)]
                }
                PlayerAction::Hit => {
                    keyframes![
                        (PlayerFrame::new(0.0), 0.0, Linear),
                        (PlayerFrame::new(6.0 - 0.001), 0.7, Linear)]
                }
                PlayerAction::HeavyHit => {
                    keyframes![
                        (PlayerFrame::new(0.0), 0.0, Linear),
                        (PlayerFrame::new(6.0 - 0.001), 0.75, Linear)]
                }
                PlayerAction::BigHit => {
                    keyframes![
                        (PlayerFrame::new(0.0), 0.0, Linear),
                        (PlayerFrame::new(8.0 - 0.001), 0.8, Linear)]
                }
                PlayerAction::Spell => {
                    keyframes![
                        (PlayerFrame::new(0.0), 0.0, Linear),
                        (PlayerFrame::new(6.0 - 0.001), 0.8, Linear)]
                }
                PlayerAction::SitDown => {
                    keyframes![
                        (PlayerFrame::new(0.0), 0.0, Linear),
                        (PlayerFrame::new(2.0 - 0.001), 1.2, Linear)]
                }
                PlayerAction::Damage => {
                    keyframes![
                        (PlayerFrame::new(0.0), 0.0, Linear),
                        (PlayerFrame::new(3.0 - 0.001), 0.7, Linear)]
                }
                PlayerAction::Die => {
                    keyframes![
                        (PlayerFrame::new(0.0), 0.0, Linear),
                        (PlayerFrame::new(4.0 - 0.001), 1.0, Linear)]
                }
            }
        }

        pub fn effect_frame(&self) -> AnimationSequence<PlayerFrame> {
            match self {
                PlayerAction::Stand => {
                    keyframes![
                        (PlayerFrame::new(0.0), 0.0, Linear),
                        (PlayerFrame::new(8.0 - 0.001), 1.5, Linear)]
                }
                PlayerAction::Walk => {
                    keyframes![
                        (PlayerFrame::new(0.0), 0.0, Linear),
                        (PlayerFrame::new(6.0 - 0.001), 0.8, Linear)]
                }
                PlayerAction::Run => {
                    keyframes![
                        (PlayerFrame::new(0.0), 0.0, Linear),
                        (PlayerFrame::new(6.0 - 0.001), 1.0, Linear)]
                }
                PlayerAction::WarMode => {
                    keyframes![
                        (PlayerFrame::new(0.0), 0.0, Linear),
                        (PlayerFrame::new(1.0 - 0.001), 1.5, Linear)]
                }
                PlayerAction::Hit => {
                    keyframes![
                        (PlayerFrame::new(0.0), 0.0, Linear),
                        (PlayerFrame::new(6.0 - 0.001), 0.7, Linear)]
                }
                PlayerAction::HeavyHit => {
                    keyframes![
                        (PlayerFrame::new(0.0), 0.0, Linear),
                        (PlayerFrame::new(6.0 - 0.001), 0.75, Linear)]
                }
                PlayerAction::BigHit => {
                    keyframes![
                        (PlayerFrame::new(0.0), 0.0, Linear),
                        (PlayerFrame::new(8.0 - 0.001), 0.7, Linear)]
                }
                PlayerAction::Spell => {
                    keyframes![
                        (PlayerFrame::new(0.0), 0.0, Linear),
                        (PlayerFrame::new(6.0 - 0.001), 0.7, Linear)]
                }
                PlayerAction::SitDown => {
                    keyframes![
                        (PlayerFrame::new(0.0), 0.0, Linear),
                        (PlayerFrame::new(2.0 - 0.001), 1.5, Linear)]
                }
                PlayerAction::Damage => {
                    keyframes![
                        (PlayerFrame::new(0.0), 0.0, Linear),
                        (PlayerFrame::new(3.0 - 0.001), 0.7, Linear)]
                }
                PlayerAction::Die => {
                    keyframes![
                        (PlayerFrame::new(0.0), 0.0, Linear),
                        (PlayerFrame::new(4.0 - 0.001), 1.0, Linear)]
                }
            }
        }
        pub fn step(&self) -> f32 {
            match self {
                PlayerAction::Stand => {4.0}
                PlayerAction::Walk => {6.0}
                PlayerAction::Run => {6.0}
                PlayerAction::WarMode => {1.0}
                PlayerAction::Hit => {6.0}
                PlayerAction::HeavyHit => {6.0}
                PlayerAction::BigHit => {8.0}
                // PlayerAction::FireHitReady => {6.0}
                PlayerAction::Spell => {6.0}
                PlayerAction::SitDown => {2.0}
                PlayerAction::Damage => {3.0}
                PlayerAction::Die => {4.0}
            }
        }

        pub fn effect_step(&self) -> f32 {
            match self {
                PlayerAction::Stand => {8.0}
                PlayerAction::Walk => {6.0}
                PlayerAction::Run => {6.0}
                PlayerAction::WarMode => {1.0}
                PlayerAction::Hit => {6.0}
                PlayerAction::HeavyHit => {6.0}
                PlayerAction::BigHit => {8.0}
                // PlayerAction::FireHitReady => {6.0}
                PlayerAction::Spell => {6.0}
                PlayerAction::SitDown => {2.0}
                PlayerAction::Damage => {3.0}
                PlayerAction::Die => {4.0}
            }
        }

        pub fn jump(&self) -> f32 {
            match self {
                PlayerAction::Stand => {0.0}
                PlayerAction::Walk => {32.0}
                PlayerAction::Run => {80.0}
                PlayerAction::WarMode => {128.0}
                PlayerAction::Hit => {136.0}
                PlayerAction::HeavyHit => {184.0}
                PlayerAction::BigHit => {232.0}
                // PlayerAction::FireHitReady => {296.0}
                PlayerAction::Spell => {296.0}
                PlayerAction::SitDown => {344.0}
                PlayerAction::Damage => {360.0}
                PlayerAction::Die => {384.0}
            }
        }

        pub fn effect_jump(&self) -> f32 {
            match self {
                PlayerAction::Stand => {0.0}
                PlayerAction::Walk => {64.0}
                PlayerAction::Run => {80.0 + 32.0}
                PlayerAction::WarMode => {128.0 + 32.0}
                PlayerAction::Hit => {136.0 + 32.0}
                PlayerAction::HeavyHit => {184.0 + 32.0}
                PlayerAction::BigHit => {232.0 + 32.0}
                // PlayerAction::FireHitReady => {296.0}
                PlayerAction::Spell => {296.0 + 32.0}
                PlayerAction::SitDown => {344.0 + 32.0}
                PlayerAction::Damage => {360.0 + 32.0}
                PlayerAction::Die => {384.0 + 32.0}
            }
        }
    }

    pub struct PlayerAnimation {
        pub file: u16,
        pub number: u16,
        pub offset: u32,
        state: PlayerAction,
        dir: Direction,
        frame: AnimationSequence<PlayerFrame>,
        hair: AnimationSequence<PlayerFrame>,
        effect: AnimationSequence<PlayerFrame>,
    }

    impl PlayerAnimation {

        pub fn new(file: u16, number: u16, offset: u32, state: PlayerAction, dir: Direction) -> Self {
            let frame: AnimationSequence<PlayerFrame> = state.new_frame();
            let hair: AnimationSequence<PlayerFrame> = state.new_frame();
            let effect: AnimationSequence<PlayerFrame> = state.effect_frame();
            Self {file, number, offset, state, dir, frame, hair, effect}
        }

        pub fn advance(&mut self, duration: f64) {
            self.frame.advance_and_maybe_wrap(duration);
            self.hair.advance_and_maybe_wrap(duration);
            self.effect.advance_and_maybe_wrap(duration);
        }

        pub fn state(&mut self, state: PlayerAction) {
            self.frame = state.new_frame();
            self.hair = state.new_frame();
            self.effect = state.effect_frame();
            self.state = state;
        }

        pub fn dir(&mut self, dir: Direction) {
            self.dir = dir;
        }

        pub fn now(&mut self) -> f32 {
            let frame = &self.frame.now();
            frame.count + self.offset as f32 + self.state.step() * self.dir.offset() + self.state.jump()
        }

        pub fn hair(&self) -> f32 {
            let count = self.hair.now().count;
            self.state.step() * self.dir.offset() + self.state.jump() + count + 1.0
        }

        pub fn effect(&self) -> f32 {
            let count = self.effect.now().count;
            self.state.effect_step() * self.dir.offset() + self.state.effect_jump() + count + 1.0
        }
    }
}

mod math {
    // (p2.y - p1.y).atan2(p2.x - p1.x) * 180.0 / std::f32::consts::PI; // 角度

    // ((p2.x - p1.x).abs().powi(2) + (p2.y - p1.y).abs().powi(2)).sqrt(); // 距离

    use ggez::mint::Point2;

    fn angle(src_x: f32, src_y: f32, dst_x: f32, dst_y: f32) -> f32 {
        (dst_y - src_y).atan2(dst_x - src_x) * 180.0 / std::f32::consts::PI
    }

    fn sharing(angle: f32, sharing: u32) -> f32 {
        // println!("angle1: {}", angle);
        let sub = 90.0 + 360.0 / 2.0 / sharing as f32 + angle;
        let angle = if sub < 0.0 { sub + 360.0 } else { sub };
        // println!("angle2: {}, sub: {}", angle, sub);
        let sub = 360.0 / sharing as f32;
        for s in 0..sharing {
            if angle >= s as f32 * sub && angle < (s as f32 + 1.0) * sub {
                return s as f32 + 1.0;
            }
        }
        return sub;
    }

    pub fn angle8(src_x: f32, src_y: f32, dst_x: f32, dst_y: f32) -> f32 {
        // let angle = angle(src, dst);
        return sharing(angle(src_x, src_y, dst_x, dst_y), 8);
    }

    pub fn angle12(src_x: f32, src_y: f32, dst_x: f32, dst_y: f32) -> f32 {
        return sharing(angle(src_x, src_y, dst_x, dst_y), 12);
    }

    pub fn angle16(src_x: f32, src_y: f32, dst_x: f32, dst_y: f32) -> f32 {
        return sharing(angle(src_x, src_y, dst_x, dst_y), 16);
    }

    pub fn distance(src_x: f32, src_y: f32, dst_x: f32, dst_y: f32) -> f32 {
        ((dst_x - src_x).abs().powi(2) + (dst_y - src_y).abs().powi(2)).sqrt()
    }
}