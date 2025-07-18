#![warn(clippy::pedantic)]
#![warn(clippy::style)]
#![deny(clippy::perf)]
#![allow(dead_code)]
#![forbid(
    clippy::missing_safety_doc,
    clippy::undocumented_unsafe_blocks,
    clippy::multiple_unsafe_ops_per_block
)]
#![warn(clippy::unnecessary_safety_doc, clippy::unnecessary_safety_comment)]
#![feature(const_trait_impl, new_range_api, unchecked_shifts, const_ops)]

use coords::FactoryVector3;
use ordinals::Cardinal2D;
use raylib::prelude::*;

mod coords;
use crate::{
    coords::{PlayerVector3, RailVector3},
    input::Bindings,
};

mod ordinals;

mod input;

mod player;
use player::Player;

mod chem;

mod factory;
use factory::{Factory, Reactor, Resources};

pub const FORWARD: Vector3 = Vector3::NEG_Z;
pub const BACKWARD: Vector3 = Vector3::Z;
pub const RIGHT: Vector3 = Vector3::X;
pub const LEFT: Vector3 = Vector3::NEG_X;
pub const UP: Vector3 = Vector3::Y;
pub const DOWN: Vector3 = Vector3::NEG_Y;

fn main() {
    #[allow(unused_imports)]
    use {KeyboardKey::*, MouseButton::*};

    let (mut rl, thread) = init().title("factory game").resizable().msaa_4x().build();

    rl.set_target_fps(60);
    rl.maximize_window();

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

    let bindings = Bindings::default_binds();

    let mut player = Player::spawn(
        &mut rl,
        &thread,
        PlayerVector3::new(0, 0, 0),
        0.0,
        0.0,
        45.0,
    );

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
        let inputs = bindings.get(&rl);
        player.update(&mut rl, &thread, &inputs, &mut factory);

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        {
            let mut d = d.begin_mode3D(player.camera);
            factory.draw(&mut d, &thread, &mut resources, &player.position);
        }

        d.draw_fps(0, 0);
        d.draw_text_ex(
            &font,
            &format!(
                "player position: ({:X}, {:X}, {:X})\n\
                player velocity: ({:X}, {:X}, {:X})\n\
                player direction: ({}, {})",
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
