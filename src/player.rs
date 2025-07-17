use crate::{
    coords::PlayerVector3,
    factory::Factory,
    input::{EventInput, Inputs, VectorInput},
};
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
    #[allow(
        clippy::unused_self,
        reason = "Should be able to use this method even if height becomes a variable"
    )]
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
        inputs: &Inputs,
        _current_factory: &mut Factory,
    ) {
        #[allow(
            unused_imports,
            clippy::enum_glob_use,
            reason = "Variants are prefixed"
        )]
        use {KeyboardKey::*, MouseButton::*};

        let dt = rl.get_frame_time();

        // Movement
        {
            // Measured in meters per second
            let move_speed = if inputs[EventInput::Sprint] {
                self.run_speed
            } else {
                self.walk_speed
            };

            let movement = inputs[VectorInput::Walk];
            self.velocity = PlayerVector3::from_vec3(
                Vector3::new(movement.x, 0.0, -movement.y) * move_speed * dt,
            );
            self.position += self.velocity;
        }
    }
}
