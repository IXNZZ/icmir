use ggez::{Context, GameError};
use crate::scene::{GameState, Scene, SceneHandler};

pub struct PlayScene {

}

impl PlayScene {
    pub fn new(_ctx: &mut Context) -> Self{
        PlayScene {}
    }
}

impl SceneHandler for PlayScene {
    fn init(&mut self, _ctx: &mut Context, state: &mut GameState) -> Result<(), GameError> {
        Ok(())
    }


    fn update(&mut self, _ctx: &mut Context) -> Result<Option<(Scene, Box<dyn SceneHandler>)>, GameError> {
        // self.proxy.switch_scene()
        Ok(None)
    }

    fn draw(&mut self, _ctx: &mut Context) -> Result<(), GameError> {
        Ok(())
    }
}