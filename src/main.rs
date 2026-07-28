mod load_models;
use load_models::load_models;
use raylib::prelude::*;

fn main() {
    const WIDTH: i32 = 1000;
    const HEIGHT: i32 = 600;
    let (mut rl, thread): (RaylibHandle, RaylibThread) = raylib::init()
        .size(WIDTH, HEIGHT)
        .title("Endless Runner")
        .build();

    let game_models = load_models(&mut rl, &thread);

    let mut camera = Camera3D {
        position: Vector3::new(5.0, 5.0, 5.0),
        target:   Vector3::new(0.0, 0.0, 0.0),
        up:       Vector3::new(0.0, 1.0, 0.0),
        fovy:     45.0,
        projection: CameraProjection::CAMERA_PERSPECTIVE,
    };

    rl.disable_cursor();

    while !rl.window_should_close() {
        let mut d: RaylibDrawHandle = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        {
            let mut d3: RaylibMode3D<RaylibDrawHandle> = d.begin_mode3D(camera);
            d3.draw_grid(20, 1.0);
        }
    }
}