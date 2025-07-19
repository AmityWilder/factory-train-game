use std::f32::consts::PI;
use crate::{
    DOWN, FORWARD, RIGHT, UP,
    coords::{PlayerCoord, PlayerVector3},
    factory::Factory,
    input::{self, Inputs},
};
use raylib::prelude::{
    glam::{EulerRot, Quat},
    *,
};

/// Meters per second per second
const GRAVITY: f32 = 9.807;

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
    let camera_offset = UP * 1.6;
    let rot = Quat::from_euler(EulerRot::XYZ, pitch, yaw, 0.0);
    (camera_offset, camera_offset + rot.mul_vec3(FORWARD))
}

impl Player {
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
            velocity: PlayerVector3::new(0, 0, 0),
            yaw,
            pitch,
            is_running: false,
            camera: Camera3D::perspective(
                camera_offset,
                camera_target,
                UP,
                fovy,
            ),
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
        #[allow(unused_imports, clippy::enum_glob_use, reason = "no reason")]
        use input::{AxisInput::*, EventInput::*, VectorInput::*};

        let dt = rl.get_frame_time();

        // Looking around
        {
            let pan = inputs[Look] * 0.0007;
            self.yaw += pan.x;
            self.pitch += pan.y;
            self.pitch = self.pitch.clamp(-PI, PI);
            self.yaw = self.yaw % (PI * 2.0);
            (self.camera.position, self.camera.target) = camera_helper(self.pitch, self.yaw);
        }
        
        // Movement
        {
            const FLOOR_HEIGHT: PlayerCoord = PlayerCoord::from_i32(0);

            let is_on_floor = self.position.y <= FLOOR_HEIGHT;
            if is_on_floor {
                self.velocity.y = 0.into();
                self.position.y = FLOOR_HEIGHT;
            }

            let mut force = PlayerVector3::new(0, 0, 0);

            // convert from polar coords, making a unit vector for the facing angle.
            let move_dir = Vector2::new(self.yaw.cos(), self.yaw.sin());
            let movement = inputs[Walk].normalize_or_zero().rotate(move_dir);
            if is_on_floor {
                if movement.length_squared() < 0.01 {
                    self.velocity -= self.velocity.scale((0.1).into());
                }
            } else {
                force += (DOWN * GRAVITY).into();
            }

            // Measured in meters per second
            let move_speed = if inputs[Sprint] {
                self.run_speed()
            } else {
                self.walk_speed()
            };

            if inputs[Jump] && is_on_floor {
                force += (UP * GRAVITY * 40.0).into();
            }

            let movement_force =
                ((RIGHT * movement.x + FORWARD * movement.y) * move_speed * 6.0).into();
            force += movement_force;

            self.velocity += force.scale(dt.into());

            let vel_len_sq = self.velocity.length_squared();
            if vel_len_sq < 0.0001.into() {
                // velocity dead zone
                self.velocity = PlayerVector3::new(0, 0, 0);
            } else {
                // quadratic friction for soft speed cap
                self.velocity *= PlayerCoord::from(1) - vel_len_sq * PlayerCoord::from_f32(0.0005);
            }

            self.position += self.velocity.scale(dt.into());
        }
    }

    #[allow(clippy::unused_self, reason = "may be used in future")]
    const fn walk_speed(&self) -> f32 { 2.2 }
    #[allow(clippy::unused_self, reason = "may be used in future")]
    const fn run_speed(&self) -> f32 { 8.6 }
}
