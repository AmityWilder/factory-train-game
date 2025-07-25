use crate::{
    input::{self, Inputs},
    math::coords::{
        VectorConstants,
        player::{PlayerCoord, PlayerVector3},
    },
    region::Region,
};
use raylib::prelude::{
    glam::{EulerRot, Quat},
    *,
};
use std::{f32::consts::PI, time::Instant};

/// Meters per second per second
const GRAVITY: PlayerCoord = PlayerCoord::from_f32(9.807);
const JUMP_DURATION: PlayerCoord = PlayerCoord::from_f32(40.0);
const FRICTION: PlayerCoord = PlayerCoord::from_f32(0.0005);
const AIR_MOBILITY_FACTOR: f32 = 0.1;

pub struct Player {
    /// Meters
    pub position: PlayerVector3,
    /// Meters per second
    pub velocity: PlayerVector3,
    pub pitch: f32,
    pub yaw: f32,
    pub is_running: bool,
    pub camera: Camera3D,
    pub region_last_changed: Instant,
}

#[inline]
fn camera_helper(pitch: f32, yaw: f32) -> (Vector3, Vector3) {
    let camera_offset = Vector3::UP * Player::EYE_HEIGHT;
    let rot = Quat::from_euler(EulerRot::YXZ, yaw, pitch, 0.0);
    (
        camera_offset,
        camera_offset + rot.mul_vec3(Vector3::FORWARD),
    )
}

impl Player {
    pub const HEIGHT: f32 = 1.75;
    pub const EYE_HEIGHT: f32 = Self::HEIGHT - 0.15;

    /// Spawn the player at the specified location
    pub fn spawn(
        _rl: &mut RaylibHandle,
        _thread: &RaylibThread,
        position: PlayerVector3,
        yaw: f32,
        pitch: f32,
        fovy: f32,
    ) -> Self {
        let (camera_offset, camera_target) = camera_helper(pitch, yaw);
        Self {
            position,
            velocity: PlayerVector3::ZERO,
            yaw,
            pitch,
            is_running: false,
            camera: Camera3D::perspective(camera_offset, camera_target, Vector3::UP, fovy),
            region_last_changed: Instant::now(),
        }
    }

    /// Tick the player (handle movement)
    pub fn do_movement(
        &mut self,
        rl: &mut RaylibHandle,
        _thread: &RaylibThread,
        inputs: &Inputs,
        current_region: &dyn Region,
    ) {
        #[allow(unused_imports, clippy::enum_glob_use, reason = "no reason")]
        use input::{AxisInput::*, EventInput::*, VectorInput::*};

        let dt = rl.get_frame_time();

        // Looking around
        {
            let pan = -inputs[Look];
            self.yaw += pan.x;
            self.yaw %= 2.0 * PI;
            self.pitch += pan.y;
            self.pitch = self.pitch.clamp(-PI, PI);
            (self.camera.position, self.camera.target) = camera_helper(self.pitch, self.yaw);
        }

        // Movement
        {
            const WORLD_FLOOR_HEIGHT: PlayerCoord = PlayerCoord::from_i32(0);

            let local_floor = current_region
                .local_floor(self)
                .unwrap_or(WORLD_FLOOR_HEIGHT);

            let is_on_floor = self.position.y <= local_floor;
            if is_on_floor {
                self.velocity.y = PlayerCoord::ZERO;
                self.position.y = local_floor;
            }

            let mut force = PlayerVector3::ZERO;

            // convert from polar coords, making a unit vector for the facing angle.
            let move_dir = Vector2::from_angle(self.yaw);
            let mut movement = inputs[Walk].normalize_or_zero().rotate(move_dir);
            if is_on_floor {
                if movement.length_squared() < 0.01 {
                    self.velocity -= self.velocity.scale(PlayerCoord::from_f32(0.1));
                }
            } else {
                force += PlayerVector3::from_vec3(Vector3::DOWN) * GRAVITY;
                movement *= AIR_MOBILITY_FACTOR;
            }

            // Measured in meters per second
            let move_speed = if inputs[Sprint] {
                self.run_speed()
            } else {
                self.walk_speed()
            };

            if inputs[Jump] && is_on_floor {
                force += PlayerVector3::from_vec3(Vector3::UP) * GRAVITY * JUMP_DURATION;
            }

            let movement_force =
                ((Vector3::RIGHT * movement.x + Vector3::FORWARD * movement.y) * move_speed * 6.0)
                    .into();
            force += movement_force;

            self.velocity += force.scale(PlayerCoord::from_f32(dt));

            let vel_len_sq = self.velocity.length_sqr();
            if vel_len_sq < PlayerCoord::from_f32(0.0001) {
                // velocity dead zone
                self.velocity = PlayerVector3::ZERO;
            } else if is_on_floor {
                // quadratic friction for soft speed cap
                self.velocity *= PlayerCoord::ONE - vel_len_sq * FRICTION;
            }

            self.position += self.velocity.scale(PlayerCoord::from_f32(dt));
        }
    }

    /// Tick player actions
    pub fn do_actions(
        &self,
        _rl: &mut RaylibHandle,
        _thread: &RaylibThread,
        _inputs: &Inputs,
        _current_region: &mut dyn Region,
    ) {
        _ = self.pitch.sin();
    }

    pub const fn eye_pos(&self) -> PlayerVector3 {
        self.position
            .plus(PlayerVector3::UP.scale(PlayerCoord::from_f32(Self::EYE_HEIGHT)))
    }

    pub fn vision_ray(&self) -> Ray {
        Ray {
            position: self.eye_pos().to_vec3(),
            direction: (self.camera.target - self.camera.position).normalize_or(Vector3::FORWARD),
        }
    }

    #[allow(clippy::unused_self, reason = "may be used in future")]
    const fn walk_speed(&self) -> f32 {
        2.2
    }
    #[allow(clippy::unused_self, reason = "may be used in future")]
    const fn run_speed(&self) -> f32 {
        8.6
    }
}
