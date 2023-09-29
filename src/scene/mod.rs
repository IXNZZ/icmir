use bytes::Bytes;
use crossbeam::channel::{Receiver, Sender};
use ggez::{Context, GameError, GameResult};
use ggez::event::{ErrorOrigin, EventHandler, MouseButton};
use ggez::input::keyboard::{KeyCode, KeyInput};
use tracing::debug;
use crate::network::create_network;

mod play;

pub trait SceneHandler<E = GameError>
    where
        E: std::fmt::Debug,
{
    fn init(&mut self, _ctx: &mut Context, state: &mut GameState) -> Result<(), E>;
    /// Called upon each logic update to the game.
    /// This should be where the game's logic takes place.
    fn update(&mut self, _ctx: &mut Context) -> Result<Option<(Scene, Box<dyn SceneHandler>)>, E>;

    /// Called to do the drawing of your game.
    /// You probably want to start this with
    /// [`Canvas::from_frame`](../graphics/struct.Canvas.html#method.from_frame) and end it
    /// with [`Canvas::finish`](../graphics/struct.Canvas.html#method.finish).
    fn draw(&mut self, _ctx: &mut Context) -> Result<(), E>;

    /// A mouse button was pressed
    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        _button: MouseButton,
        _x: f32,
        _y: f32,
    ) -> Result<(), E> {
        Ok(())
    }

    /// A mouse button was released
    fn mouse_button_up_event(
        &mut self,
        _ctx: &mut Context,
        _button: MouseButton,
        _x: f32,
        _y: f32,
    ) -> Result<(), E> {
        Ok(())
    }

    /// The mouse was moved; it provides both absolute x and y coordinates in the window,
    /// and relative x and y coordinates compared to its last position.
    fn mouse_motion_event(
        &mut self,
        _ctx: &mut Context,
        _x: f32,
        _y: f32,
        _dx: f32,
        _dy: f32,
    ) -> Result<(), E> {
        Ok(())
    }

    /// mouse entered or left window area
    fn mouse_enter_or_leave(&mut self, _ctx: &mut Context, _entered: bool) -> Result<(), E> {
        Ok(())
    }

    /// The mousewheel was scrolled, vertically (y, positive away from and negative toward the user)
    /// or horizontally (x, positive to the right and negative to the left).
    fn mouse_wheel_event(&mut self, _ctx: &mut Context, _x: f32, _y: f32) -> Result<(), E> {
        Ok(())
    }

    /// A keyboard button was pressed.
    ///
    /// The default implementation of this will call [`ctx.request_quit()`](crate::Context::request_quit)
    /// when the escape key is pressed. If you override this with your own
    /// event handler you have to re-implement that functionality yourself.
    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        input: KeyInput,
        _repeated: bool,
    ) -> Result<(), E> {
        if input.keycode == Some(KeyCode::Escape) {
            ctx.request_quit();
        }
        Ok(())
    }

    /// A keyboard button was released.
    fn key_up_event(&mut self, _ctx: &mut Context, _input: KeyInput) -> Result<(), E> {
        Ok(())
    }

    /// A unicode character was received, usually from keyboard input.
    /// This is the intended way of facilitating text input.
    fn text_input_event(&mut self, _ctx: &mut Context, _character: char) -> Result<(), E> {
        Ok(())
    }

    /// Called when the window is shown or hidden.
    fn focus_event(&mut self, _ctx: &mut Context, _gained: bool) -> Result<(), E> {
        Ok(())
    }

    /// Called upon a quit event.  If it returns true,
    /// the game does not exit (the quit event is cancelled).
    fn quit_event(&mut self, _ctx: &mut Context) -> Result<bool, E> {
        debug!("quit_event() callback called, quitting...");
        Ok(false)
    }

    /// Called when the user resizes the window, or when it is resized
    /// via [`GraphicsContext::set_mode()`](../graphics/struct.GraphicsContext.html#method.set_mode).
    fn resize_event(&mut self, _ctx: &mut Context, _width: f32, _height: f32) -> Result<(), E> {
        Ok(())
    }

    /// Something went wrong, causing a `GameError` (or some other kind of error, depending on what you specified).
    /// If this returns true, the error was fatal, so the event loop ends, aborting the game.
    fn on_error(&mut self, _ctx: &mut Context, _origin: ErrorOrigin, _e: E) -> bool {
        true
    }
}

pub enum Scene {
    Loading,
    Login,
    Role,
    Play,
}

pub struct GameState {
    base_dir: String,
    net_sender: Sender<Bytes>,
    net_receiver: Receiver<Bytes>
}

pub struct MainScene {
    scene: Scene,
    base_dir: String,
    handle: Box<dyn SceneHandler>,
    state: GameState,
}

impl MainScene {
    pub fn new(base_dir: &str, _ctx: &mut Context) -> Self {
        let net = create_network("");
        let mut state = GameState { base_dir: base_dir.to_string(), net_sender: net.0, net_receiver: net.1 };
        let mut current_scene = Box::new(play::PlayScene::new(_ctx));
        let _ = current_scene.init(_ctx, &mut state);
        let mut proxy = Self {
            base_dir: base_dir.to_string(),
            scene: Scene::Play,
            handle: current_scene,
            state
        };

        proxy
    }

    pub fn switch_scene(&mut self, scene: Scene, handle: Box<dyn SceneHandler>, ctx: &mut Context) {
        self.scene = scene;
        self.handle = handle;
        let _ = self.handle.init(ctx, &mut self.state);
    }
}

impl EventHandler for MainScene {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        if let Ok(Some(s)) = self.handle.update(_ctx) {
            self.switch_scene(s.0, s.1, _ctx);
        }

        Ok(())
    }

    fn draw(&mut self, _ctx: &mut Context) -> GameResult {
        self.handle.draw(_ctx)
    }

    fn mouse_button_down_event(&mut self, _ctx: &mut Context, _button: MouseButton, _x: f32, _y: f32) -> GameResult {
        self.handle.mouse_button_down_event(_ctx, _button, _x, _y)
    }

    fn mouse_button_up_event(&mut self, _ctx: &mut Context, _button: MouseButton, _x: f32, _y: f32) -> GameResult {
        self.handle.mouse_button_up_event(_ctx, _button, _x, _y)
    }

    fn mouse_motion_event(&mut self, _ctx: &mut Context, _x: f32, _y: f32, _dx: f32, _dy: f32) -> GameResult {
        self.handle.mouse_motion_event(_ctx, _x, _y, _dx, _dy)
    }

    fn mouse_enter_or_leave(&mut self, _ctx: &mut Context, _entered: bool) -> GameResult {
        self.handle.mouse_enter_or_leave(_ctx, _entered)
    }

    fn mouse_wheel_event(&mut self, _ctx: &mut Context, _x: f32, _y: f32) -> GameResult {
        self.handle.mouse_wheel_event(_ctx, _x, _y)
    }

    fn key_down_event(&mut self, ctx: &mut Context, input: KeyInput, _repeated: bool) -> GameResult {
        self.handle.key_down_event(ctx, input, _repeated)
    }

    fn key_up_event(&mut self, _ctx: &mut Context, _input: KeyInput) -> GameResult {
        self.handle.key_up_event(_ctx, _input)
    }

    fn text_input_event(&mut self, _ctx: &mut Context, _character: char) -> GameResult {
        self.handle.text_input_event(_ctx, _character)
    }

    fn focus_event(&mut self, _ctx: &mut Context, _gained: bool) -> GameResult {
        self.handle.focus_event(_ctx, _gained)
    }

    fn quit_event(&mut self, _ctx: &mut Context) -> Result<bool, GameError> {
        self.handle.quit_event(_ctx)
    }

    fn resize_event(&mut self, _ctx: &mut Context, _width: f32, _height: f32) -> GameResult {
        self.handle.resize_event(_ctx, _width, _height)
    }
}