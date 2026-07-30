use raylib::prelude::*;

pub struct GameModels {
    pub player: Model,
    pub cone: Model,
    pub cart: Model,
}

pub fn load_models(rl: &mut RaylibHandle, thread: &RaylibThread) -> GameModels {

    let player = rl.load_model(thread, "assets/human.glb").unwrap();
    let cone = rl.load_model(thread, "assets/cone2.glb").unwrap();
    let cart = rl.load_model(thread, "assets/cart2.glb").unwrap();

    GameModels {
        player,
        cone,
        cart
    }
}