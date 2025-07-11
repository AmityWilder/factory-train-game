use std::assert_matches::assert_matches;
use raylib::prelude::*;
use crate::{coords::{PlayerCoord, PlayerVector3}, factory::Factory};

const WORLD_FLOOR_Y: i32 = 0;
const GRAVITY: f32 = 9.8;

pub struct Player {
    pub position: PlayerVector3,
    pub velocity: PlayerVector3,
    pub is_running: bool,
    pub walk_speed: f32,
    pub run_speed: f32,
}

impl Player {
    /// Camera height in meters
    #[inline]
    pub const fn height(&self) -> f32 {
        1.6
    }

    /// Spawn the player at the specified location
    pub fn spawn(_rl: &mut RaylibHandle, _thread: &RaylibThread, position: PlayerVector3) -> Self {
        Self {
            position,
            velocity: PlayerVector3::new(0, 0, 0),
            is_running: false,
            walk_speed: 2.2,
            run_speed: 8.6,
        }
    }

    /// Tick the player (handle movement and actions)
    pub fn update(&mut self, rl: &mut RaylibHandle, _thread: &RaylibThread, _current_factory: &mut Factory) {
        #[allow(unused_imports)]
        use {KeyboardKey::*, MouseButton::*};

        let dt = rl.get_frame_time();

        let on_ground = self.position.y.to_f32() as i32 <= WORLD_FLOOR_Y;

        if !on_ground {
            self.velocity.y -= PlayerCoord::from_f32(GRAVITY);
        } else {
            if self.velocity.y < PlayerCoord::from_i32(0) {
                self.velocity.y = 0.into();
            }
        }

        // Movement
        {
            if on_ground && rl.is_key_pressed(KEY_SPACE) {
                self.velocity.y = PlayerCoord::from_i32(5);
            }

            self.is_running = rl.is_key_down(KEY_LEFT_SHIFT);

            // Measured in meters per second
            let move_speed = if self.is_running { self.run_speed } else { self.walk_speed };

            let movement_forward = (rl.is_key_down(KEY_S) as i8 - rl.is_key_down(KEY_W) as i8) as f32 * move_speed; 
            let movement_right = (rl.is_key_down(KEY_D) as i8 - rl.is_key_down(KEY_A) as i8) as f32 * move_speed;
            self.velocity.x = movement_right.into();
            self.velocity.z = movement_forward.into();
            assert_matches!(self.velocity.z.to_f32(), -100.0..100.0);
            self.position += self.velocity * PlayerCoord::from(dt);
        }
    }
}
