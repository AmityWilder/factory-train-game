use crate::{player::Player, resource::Resources};
use raylib::prelude::*;

fn draw_skybox(_d: &mut impl RaylibDraw3D, _thread: &RaylibThread, resources: &Resources) {
    #[allow(
        clippy::cast_possible_wrap,
        reason = "RL_QUADS is an i32 in Raylib, but bindgen made it a u32"
    )]
    const RL_QUADS: i32 = ffi::RL_QUADS as i32;

    #[allow(
        clippy::multiple_unsafe_ops_per_block,
        reason = "safety comment is complicated and shared by all operations in this block"
    )]
    // SAFETY: RaylibDraw3D is exclusively borrowed, guaranteeing the window has been
    // initialized, 3D drawing processes are loaded, and rlgl statics are syncronous
    // for this function (assuming no soundness holes outside of this function).
    // RaylibThread (which does not implement Send/Sync) is borrowed, guaranteeing
    // this is the thread that initialized the window and graphics.
    unsafe {
        ffi::rlSetTexture(resources.skybox.id);
        ffi::rlBegin(RL_QUADS);
        {
            ffi::rlColor4ub(255, 255, 255, 255);

            ffi::rlTexCoord2f(0.0, 1.0);
            ffi::rlVertex3f(-1000.0, 50.0, -1000.0);

            ffi::rlTexCoord2f(1.0, 1.0);
            ffi::rlVertex3f(1000.0, 50.0, -1000.0);

            ffi::rlTexCoord2f(1.0, 0.0);
            ffi::rlVertex3f(1000.0, 50.0, 1000.0);

            ffi::rlTexCoord2f(0.0, 0.0);
            ffi::rlVertex3f(-1000.0, 50.0, 1000.0);
        }
        ffi::rlEnd();
        ffi::rlSetTexture(0);
    }
}

pub struct World;

impl World {
    #[allow(clippy::unused_self, reason = "trait-like")]
    pub fn draw(
        &self,
        d: &mut impl RaylibDraw3D,
        thread: &RaylibThread,
        resources: &Resources,
        _player: &Player,
    ) {
        d.draw_plane(
            Vector3::ZERO,
            Vector2::new(1000.0, 1000.0),
            Color::DARKGREEN,
        );
        draw_skybox(d, thread, resources);
    }
}
