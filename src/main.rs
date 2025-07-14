#![allow(dead_code)]
#![forbid(
    clippy::missing_safety_doc,
    clippy::undocumented_unsafe_blocks,
    clippy::multiple_unsafe_ops_per_block,
)]
#![feature(assert_matches)]

use raylib::prelude::*;

mod coords;
use crate::{coords::*, ordinals::Cardinal2D};

mod ordinals;

mod player;
use player::Player;

mod chem;

mod factory;
use factory::*;

fn main() {
    #[allow(unused_imports)]
    use {KeyboardKey::*, MouseButton::*};

    let (mut rl, thread) = init()
        .title("factory game")
        .resizable()
        .msaa_4x()
        .build();

    rl.set_target_fps(60);
    rl.maximize_window();

    let font = rl.load_font_from_memory(&thread, ".ttf", include_bytes!("../assets/FiraCode-Regular.ttf"), 20, None).unwrap();

    let mut player = Player::spawn(&mut rl, &thread, PlayerVector3::new(0, 0, 0));

    let mut factory: Factory = Factory {
        origin: RailVector3 { x: 0, y: 0, z: -10 },
        machines: vec![
            Machine::Reactor(Reactor {
                position: FactoryVector3 { x: 5, y: 0, z: -4 },
                rotation: Cardinal2D::North,
            }),
            Machine::Reactor(Reactor {
                position: FactoryVector3 { x: -5, y: 2, z: -4 },
                rotation: Cardinal2D::North,
            }),
        ],
    };

    while !rl.window_should_close() {
        player.update(&mut rl, &thread, &mut factory);

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        {
            let mut d = d.begin_mode3D(Camera3D::perspective(
                Vector3::new(0.0, player.height(), 0.0),
                Vector3::new(0.0, player.height(), -1.0),
                Vector3::new(0.0, 1.0, 0.0),
                45.0,
            ));
            factory.draw(&mut d, &thread, &player.position);
        }

        d.draw_fps(0, 0);
        d.draw_text_ex(
            &font,
            &format!(
                "player position: ({:X}, {:X}, {:X})",
                player.position.x,
                player.position.y,
                player.position.z,
            ),
            Vector2::new(0.0, 20.0),
            20.0,
            0.0,
            Color::MAGENTA,
        );
    }
}
