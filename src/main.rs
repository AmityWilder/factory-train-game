#![allow(dead_code)]
#![forbid(
    clippy::missing_safety_doc,
    clippy::undocumented_unsafe_blocks,
    clippy::multiple_unsafe_ops_per_block,
)]

use raylib::prelude::*;

mod coords;
use crate::coords::*;

mod player;
use player::Player;

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

    rl.maximize_window();

    let _font = rl.load_font_from_memory(&thread, ".ttf", include_bytes!("./FiraCode-Regular.ttf"), 20, None).unwrap();

    let mut player = Player::spawn(&mut rl, &thread, PlayerVector3::new(0, 0, 0));

    let mut factory: Factory = Factory {
        origin: RailVector3 { x: 0, y: 0, z: -10 },
        machines: vec![
            Machine::Reactor(Reactor {
                position: FactoryVector3 { x: 5, y: 0, z: -4 },
                rotation: Cardinal2D::N,
            }),
            Machine::Reactor(Reactor {
                position: FactoryVector3 { x: -5, y: 2, z: -4 },
                rotation: Cardinal2D::N,
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
    }
}
