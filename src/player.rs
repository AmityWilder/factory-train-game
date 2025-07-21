use crate::{
    DOWN, FORWARD, RIGHT, UP,
    coords::{PlayerCoord, PlayerVector3},
    factory::{Clearance, Factory, Machine},
    input::{self, Inputs},
};
use raylib::prelude::{
    glam::{EulerRot, Quat},
    *,
};
use std::f32::consts::PI;

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
}

#[inline]
fn camera_helper(pitch: f32, yaw: f32) -> (Vector3, Vector3) {
    let camera_offset = UP * Player::EYELEVEL;
    let rot = Quat::from_euler(EulerRot::YXZ, yaw, pitch, 0.0);
    (camera_offset, camera_offset + rot.mul_vec3(FORWARD))
}

impl Player {
    pub const HEIGHT: f32 = 1.75;
    pub const EYELEVEL: f32 = Self::HEIGHT - 0.15;

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
            camera: Camera3D::perspective(camera_offset, camera_target, UP, fovy),
        }
    }

    /// Tick the player (handle movement and actions)
    pub fn update(
        &mut self,
        rl: &mut RaylibHandle,
        _thread: &RaylibThread,
        inputs: &Inputs,
        current_factory: &mut Factory,
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

            let position_in_factory = self.position.to_factory_relative(current_factory.origin);

            let local_floor = current_factory
                .reactors
                .iter()
                .filter_map(|reactor| {
                    let bounds = reactor.bounding_box();
                    (bounds.x.contains(&position_in_factory.x)
                        && bounds.z.contains(&position_in_factory.z)
                        // Add some extra height so that the floor doesn't reset to default after moving the player on top
                        && (bounds.y.start..=bounds.y.end).contains(&position_in_factory.y)
                        // Don't teleport up more than a meter
                        && position_in_factory.y.abs_diff(bounds.y.end) <= 1)
                        .then_some(bounds.y.end)
                })
                .max()
                .map_or(WORLD_FLOOR_HEIGHT, |y| PlayerCoord::from_i32(y.into()));

            let is_on_floor = self.position.y <= local_floor;
            if is_on_floor {
                self.velocity.y = 0.into();
                self.position.y = local_floor;
            }

            let mut force = PlayerVector3::ZERO;

            // convert from polar coords, making a unit vector for the facing angle.
            let move_dir = Vector2::from_angle(self.yaw);
            let mut movement = inputs[Walk].normalize_or_zero().rotate(move_dir);
            if is_on_floor {
                if movement.length_squared() < 0.01 {
                    self.velocity -= self.velocity.scale((0.1).into());
                }
            } else {
                force += PlayerVector3::from_vec3(DOWN) * GRAVITY;
                movement *= AIR_MOBILITY_FACTOR;
            }

            // Measured in meters per second
            let move_speed = if inputs[Sprint] {
                self.run_speed()
            } else {
                self.walk_speed()
            };

            if inputs[Jump] && is_on_floor {
                force += PlayerVector3::from_vec3(UP) * GRAVITY * JUMP_DURATION;
            }

            let movement_force =
                ((RIGHT * movement.x + FORWARD * movement.y) * move_speed * 6.0).into();
            force += movement_force;

            self.velocity += force.scale(dt.into());

            let vel_len_sq = self.velocity.length_squared();
            if vel_len_sq < 0.0001.into() {
                // velocity dead zone
                self.velocity = PlayerVector3::ZERO;
            } else if is_on_floor {
                // quadratic friction for soft speed cap
                self.velocity *= PlayerCoord::ONE - vel_len_sq * FRICTION;
            }

            self.position += self.velocity.scale(dt.into());
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
