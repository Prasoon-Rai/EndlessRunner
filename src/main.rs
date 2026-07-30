mod load_models;
use raylib::consts::CameraMode::CAMERA_FREE;
use load_models::load_models;
use raylib::prelude::*;

fn main() {
    const WIDTH: i32 = 1500;
    const HEIGHT: i32 = 800;

    let (mut rl, thread): (RaylibHandle, RaylibThread) = raylib::init()
        .size(WIDTH, HEIGHT)
        .title("Endless Runner")
        .build();

    let mut player_pos = Vector3::new(0.0, 0.0, 0.0);
    let player_speed: f32 = 5.0;

    // jump state
    let ground_y: f32 = 0.0;
    let mut velocity_y: f32 = 0.0;
    let gravity: f32 = -25.0;
    let jump_force: f32 = 10.0;
    let mut is_grounded = true;

    let camera_offset = Vector3::new(0.0, 3.2, -7.0);

    let mut camera = Camera3D {
        position: Vector3::new(
            player_pos.x + camera_offset.x,
            player_pos.y + camera_offset.y,
            player_pos.z + camera_offset.z,
        ),
        target: player_pos,
        up: Vector3::new(0.0, 1.0, 0.0),
        fovy: 45.0,
        projection: CameraProjection::CAMERA_PERSPECTIVE,
    };

    let game_models = load_models(&mut rl, &thread);

    rl.disable_cursor();

    while !rl.window_should_close() {
        let frame_time = rl.get_frame_time();

        let jump_pressed = rl.is_key_pressed(KeyboardKey::KEY_SPACE);

        if jump_pressed && is_grounded {
            velocity_y = jump_force;
            is_grounded = false;
        }

        velocity_y += gravity * frame_time;
        player_pos.y += velocity_y * frame_time;

        if player_pos.y <= ground_y {
            player_pos.y = ground_y;
            velocity_y = 0.0;
            is_grounded = true;
        }

        player_pos.z += player_speed * frame_time;

        // --- camera follow ---
        camera.position = Vector3::new(
            player_pos.x + camera_offset.x,
            player_pos.y + camera_offset.y,
            player_pos.z + camera_offset.z,
        );
        camera.target = player_pos;

        let mut d: RaylibDrawHandle = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        {
            let mut d3: RaylibMode3D<RaylibDrawHandle> = d.begin_mode3D(camera);
            d3.draw_grid(40, 2.0);
            d3.draw_model(&game_models.player, player_pos, 1.0, Color::WHITE);
        }
    }
}