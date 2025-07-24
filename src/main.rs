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
    clippy::allow_attributes_without_reason,
    clippy::must_use_candidate
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
    const_range_bounds,
    associated_type_defaults
)]

mod chem;
mod input;
mod math;
mod ordinals;
mod player;
mod region;
mod resource;
mod rlights;

use crate::{math::bounds::FactoryBounds, region::RegionId};
use math::{
    bounds::LabBounds,
    coords::{LabVector3, VectorConstants},
};
use raylib::prelude::*;
use region::{
    Region,
    factory::{Factory, Reactor},
    lab::{Laboratory, PeriodicTable},
};
use {
    input::Bindings,
    math::coords::{factory::FactoryVector3, player::PlayerVector3, rail::RailVector3},
    ordinals::Cardinal2D,
    player::Player,
    resource::Resources,
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

#[allow(clippy::too_many_lines, reason = "don't care")]
fn main() {
    let (mut rl, thread) = init().title("factory game").resizable().msaa_4x().build();

    rl.set_target_fps(60);
    rl.maximize_window();
    rl.hide_cursor();
    rl.disable_cursor();

    let resources = Resources::new(&mut rl, &thread);

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

    let mut factories: Vec<Factory> = vec![
        Factory {
            origin: RailVector3 { x: 0, y: 0, z: 0 },
            bounds: FactoryBounds {
                min: FactoryVector3::new(-30, 0, -30),
                max: FactoryVector3::new(30, 30, 30),
            },
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
        },
        Factory {
            origin: RailVector3 {
                x: 300,
                y: 0,
                z: 50,
            },
            bounds: FactoryBounds {
                min: FactoryVector3::new(-30, 0, -30),
                max: FactoryVector3::new(30, 30, 30),
            },
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
        },
    ];

    let mut lab = Laboratory {
        origin: PlayerVector3::from_i32(5, 0, -30),
        bounds: LabBounds {
            min: LabVector3::from_i16(-10, 0, -10),
            max: LabVector3::from_i16(10, 10, 10),
        },
        periodic_table: Some(PeriodicTable {
            position: LabVector3::from_i16(0, 0, 0),
        }),
    };

    let mut current_region = RegionId::Factory(0);

    while !rl.window_should_close() {
        let inputs = bindings.check(&rl);
        player.do_movement(
            &mut rl,
            &thread,
            &inputs,
            &current_region.to_region(&factories, &lab),
        );

        current_region = RegionId::containing(&player.eye_pos(), &factories, &lab);

        player.do_actions(
            &mut rl,
            &thread,
            &inputs,
            &mut current_region.to_mut_region(&mut factories, &mut lab),
        );

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        {
            let mut d = d.begin_mode3D(player.camera);
            let player_pos = &player.position;
            for factory in &factories {
                let origin = &factory.origin;
                d.draw_bounding_box(
                    BoundingBox {
                        min: factory.bounds.min.to_player_relative(player_pos, origin),
                        max: factory.bounds.max.to_player_relative(player_pos, origin),
                    },
                    Color::GREEN,
                );
            }
            d.draw_bounding_box(
                BoundingBox {
                    min: lab.bounds.min.to_player_relative(player_pos, &lab.origin),
                    max: lab.bounds.max.to_player_relative(player_pos, &lab.origin),
                },
                Color::ORANGE,
            );
            current_region
                .to_region(&factories, &lab)
                .draw(&mut d, &thread, &resources, &player);
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
