use raylib::prelude::*;

pub struct GameModels {
    pub player: Model,
    pub running_anim: raylib::models::ModelAnimations,
    pub jumping_anim: raylib::models::ModelAnimations,
    pub sliding_anim: raylib::models::ModelAnimations,
}

pub fn load_models(rl: &mut RaylibHandle, thread: &RaylibThread) -> GameModels {
    let player = rl
        .load_model(thread, "assets/running.glb")
        .unwrap();

    let running_anim = rl.load_model_animations(thread, "assets/running.glb").unwrap();
    let jumping_anim = rl.load_model_animations(thread, "assets/jumping.glb").unwrap();
    let sliding_anim = rl.load_model_animations(thread, "assets/sliding.glb").unwrap();

    GameModels {
        player,
        running_anim,
        jumping_anim,
        sliding_anim,
    }
}