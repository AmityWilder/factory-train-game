use crate::{coords::PlayerVector3, factory::Factory};
use raylib::prelude::*;

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
    pub fn update(
        &mut self,
        rl: &mut RaylibHandle,
        _thread: &RaylibThread,
        _current_factory: &mut Factory,
    ) {
        #[allow(unused_imports)]
        use {KeyboardKey::*, MouseButton::*};

        let dt = rl.get_frame_time();

        // Movement
        {
            if rl.is_key_pressed(KEY_LEFT_SHIFT) {
                self.is_running = !self.is_running;
            }

            // Measured in meters per second
            let move_speed = if self.is_running {
                self.run_speed
            } else {
                self.walk_speed
            };

            let movement_forward =
                (rl.is_key_down(KEY_S) as i8 - rl.is_key_down(KEY_W) as i8) as f32;
            let movement_right = (rl.is_key_down(KEY_D) as i8 - rl.is_key_down(KEY_A) as i8) as f32;
            self.velocity = PlayerVector3::from_vec3(
                Vector3::new(movement_right, 0.0, movement_forward) * move_speed * dt,
            );
            self.position += self.velocity;
        }
    }
}
