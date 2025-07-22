#![warn(clippy::pedantic)]
#![warn(clippy::all)]
#![warn(clippy::style)]
#![deny(clippy::perf, clippy::multiple_unsafe_ops_per_block)]
#![allow(dead_code, reason = "under development")]
#![forbid(clippy::missing_safety_doc, clippy::undocumented_unsafe_blocks)]
#![forbid(
    clippy::missing_const_for_fn,
    reason = "a const fn not marked as const denies callers the opportunity to be const"
)]
#![warn(
    clippy::unnecessary_safety_doc,
    clippy::unnecessary_safety_comment,
    clippy::allow_attributes_without_reason
)]
#![feature(
    const_trait_impl,
    new_range_api,
    unchecked_shifts,
    const_ops,
    stmt_expr_attributes,
    custom_inner_attributes,
    assert_matches,
    const_try,
    const_range_bounds
)]

use raylib::prelude::*;

mod chem;
mod coords;
mod factory;
mod input;
mod ordinals;
mod periodic_table;
mod player;
mod rlights;

use {
    coords::{FactoryVector3, PlayerVector3, RailVector3},
    factory::{Factory, Reactor, Resources},
    input::Bindings,
    ordinals::Cardinal2D,
    player::Player,
};

fn set_bindings_default(bindings: &mut Bindings) {
    #[allow(unused_imports, reason = "subject to change")]
    #[allow(clippy::enum_glob_use, reason = "ergonomics")]
    use {
        input::{
            AxisInput::{self, *},
            AxisSource::{self, *},
            EventInput::{self, *},
            EventSource::{self, *},
            KeyStateExt,
            VectorInput::{self, *},
            VectorSource::{self, *},
        },
        raylib::prelude::{GamepadAxis::*, GamepadButton::*, KeyboardKey::*, MouseButton::*},
    };

    let mouse_sensitivity = 0.001;
    let arrow_sensitivity = 0.02;

    *bindings = Bindings::default();
    bindings[Walk] = (KEY_D.down() - KEY_A.down())
        .cartesian(KEY_W.down() - KEY_S.down())
        .normalize();
    bindings[Look] = Mouse * mouse_sensitivity
        + (KEY_RIGHT.down() - KEY_LEFT.down()).cartesian(KEY_UP.down() - KEY_DOWN.down())
            * arrow_sensitivity;
    bindings[Sprint] = KEY_LEFT_SHIFT.down() | KEY_RIGHT_SHIFT.down();
    bindings[Jump] = KEY_SPACE.pressed();
    bindings[NextItem] = MouseWheel.max_magnitude().gt(0.0);
    bindings[PrevItem] = MouseWheel.max_magnitude().lt(0.0);
}

fn main() {
    let (mut rl, thread) = init().title("factory game").resizable().msaa_4x().build();

    rl.set_target_fps(60);
    rl.maximize_window();
    rl.hide_cursor();
    rl.disable_cursor();

    let mut resources = Resources::new(&mut rl, &thread);

    let font = rl
        .load_font_from_memory(
            &thread,
            ".ttf",
            include_bytes!("../assets/FiraCode-Regular.ttf"),
            20,
            None,
        )
        .unwrap();

    let mut bindings = Bindings::default_binds();

    let mut player = Player::spawn(&mut rl, &thread, PlayerVector3::ZERO, 0.0, 0.0, 45.0);

    let mut factory: Factory = Factory {
        origin: RailVector3 { x: 0, y: 0, z: 0 },
        reactors: vec![
            Reactor {
                position: FactoryVector3 { x: 5, y: 0, z: -6 },
                rotation: Cardinal2D::default(),
            },
            Reactor {
                position: FactoryVector3 { x: -3, y: 0, z: -9 },
                rotation: Cardinal2D::default(),
            },
        ],
    };

    while !rl.window_should_close() {
        let inputs = bindings.check(&rl);
        player.do_movement(&mut rl, &thread, &inputs, &factory);
        let player_vision_ray = player.vision_ray();
        player.do_actions(&mut rl, &thread, &inputs, &mut factory);

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        {
            let mut d = d.begin_mode3D(player.camera);
            let view_target = factory.get_ray_collision(player_vision_ray);
            factory.draw(
                &mut d,
                &thread,
                &mut resources,
                &player,
                view_target.as_ref(),
            );
        }

        d.draw_fps(0, 0);
        d.draw_text_ex(
            &font,
            &format!(
                "player position: ({:.3}, {:.3}, {:.3})\n\
                player velocity: ({:.3}, {:.3}, {:.3})\n\
                player direction: ({:.3}, {:.3})",
                player.position.x,
                player.position.y,
                player.position.z,
                player.velocity.x,
                player.velocity.y,
                player.velocity.z,
                player.yaw,
                player.pitch,
            ),
            Vector2::new(0.0, 20.0),
            20.0,
            0.0,
            Color::MAGENTA,
        );
    }
}
