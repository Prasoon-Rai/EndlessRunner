mod load_models;
use load_models::load_models;
use raylib::prelude::*;

#[derive(Clone, Copy, PartialEq)]
enum ObstacleKind {
    None,
    Cone,
    Cart,
}

struct Segment {
    z: f32,
    lane: i32,
    kind: ObstacleKind,
}

fn check_collision(player_pos: &Vector3, segments: &[Segment]) -> bool {
    const OBSTACLE_HALF_DEPTH: f32 = 0.6;  // half the model's actual z-length, tune to your mesh
    const OBSTACLE_HALF_WIDTH: f32 = 0.8;
    const SAFE_JUMP_HEIGHT: f32 = 1.5;
    const LANE_WIDTH: f32 = 3.0;

    for seg in segments {
        if seg.kind == ObstacleKind::None {
            continue;
        }
        
        let obstacle_x = seg.lane as f32 * LANE_WIDTH;
        let x_hit = (player_pos.x - obstacle_x).abs() < OBSTACLE_HALF_WIDTH;
        let z_hit = (player_pos.z - seg.z).abs() < OBSTACLE_HALF_DEPTH;
        let y_hit = player_pos.y < SAFE_JUMP_HEIGHT;

        if x_hit && z_hit && y_hit {
            return true;
        }
    }
    false
}
fn main() {
    const WIDTH: i32 = 1500;
    const HEIGHT: i32 = 800;
    const LANE_WIDTH: f32 = 3.0;
    const SEGMENT_LENGTH: f32 = 10.0;
    const NUM_SEGMENTS: usize = 12;

    let (mut rl, thread): (RaylibHandle, RaylibThread) = raylib::init()
        .size(WIDTH, HEIGHT)
        .title("Endless Runner")
        .build();

    let mut player_pos = Vector3::new(0.0, 0.0, 0.0);
    let player_speed: f32 = 20.0;

    let ground_y: f32 = 0.0;
    let mut velocity_y: f32 = 0.0;
    let gravity: f32 = -25.0;
    let jump_force: f32 = 10.0;
    let mut is_grounded = true;

    let mut current_lane: i32 = 0;
    let lane_snap_speed: f32 = 15.0;

    let mut game_over = false;

    let mut segments: Vec<Segment> = (0..NUM_SEGMENTS)
        .map(|i| Segment {
            z: i as f32 * SEGMENT_LENGTH,
            lane: 0,
            kind: ObstacleKind::None,
        })
        .collect();

    let mut max_z = segments.iter().map(|s| s.z).fold(f32::MIN, f32::max);

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
        let move_left = rl.is_key_pressed(KeyboardKey::KEY_D);
        let move_right = rl.is_key_pressed(KeyboardKey::KEY_A);

        if !game_over {
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

            if move_left && current_lane > -1 {
                current_lane -= 1;
            }
            if move_right && current_lane < 1 {
                current_lane += 1;
            }
            let target_x = current_lane as f32 * LANE_WIDTH;
            player_pos.x += (target_x - player_pos.x) * (lane_snap_speed * frame_time).min(1.0);

            player_pos.z += player_speed * frame_time;

            for seg in segments.iter_mut() {
                if player_pos.z - seg.z > SEGMENT_LENGTH {
                    max_z += SEGMENT_LENGTH;
                    seg.z = max_z;
                    seg.lane = rl.get_random_value::<i32>(-1..=1);
                    seg.kind = match rl.get_random_value::<i32>(0..=3) {
                        0 => ObstacleKind::None,
                        1 => ObstacleKind::Cone,
                        _ => ObstacleKind::Cart,
                    };
                }
            }

            if check_collision(&player_pos, &segments) {
                game_over = true;
            }

            camera.position = Vector3::new(
                player_pos.x + camera_offset.x,
                player_pos.y + camera_offset.y,
                player_pos.z + camera_offset.z,
            );
            camera.target = player_pos;
        }

        let mut d: RaylibDrawHandle = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        {
            let mut d3: RaylibMode3D<RaylibDrawHandle> = d.begin_mode3D(camera);
            d3.draw_grid(40, 2.0);
            d3.draw_model(&game_models.player, player_pos, 1.0, Color::WHITE);

            for seg in &segments {
                let obstacle_pos = Vector3::new(seg.lane as f32 * LANE_WIDTH, 0.0, seg.z);
                match seg.kind {
                    ObstacleKind::Cone => {
                        d3.draw_model(&game_models.cone, obstacle_pos, 1.5, Color::WHITE) // 1.5
                    }
                    ObstacleKind::Cart => {
                        d3.draw_model(&game_models.cart, obstacle_pos, 1.0, Color::WHITE) // 0.022
                    }
                    ObstacleKind::None => {}
                }
            }
        }

        if game_over {
            d.draw_text("GAME OVER", WIDTH / 2 - 100, HEIGHT / 2, 40, Color::RED);
        }
    }
}