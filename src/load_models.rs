use raylib::prelude::*;

pub struct GameModels {
    pub player: Model,
}

pub fn load_models(rl: &mut RaylibHandle, thread: &RaylibThread) -> GameModels {
    let player = rl
        .load_model(thread, "assets/human.glb")
        .unwrap();

    GameModels {
        player,
    }
}