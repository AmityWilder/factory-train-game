use crate::{
    math::coords::{FactoryVector3, PlayerVector3},
    region::factory::Factory,
    resource::Resources,
    rl_helpers::DynRaylibDraw3D,
};
use raylib::prelude::*;
use std::time::{Duration, Instant};

#[derive(Debug)]
pub struct GridVisualizer {
    pub start_time: Instant,
}

impl GridVisualizer {
    const ANIMATION_TIME: Duration = Duration::from_millis(500);

    pub fn draw(
        &self,
        d: &mut DynRaylibDraw3D,
        _thread: &RaylibThread,
        _resources: &Resources,
        player_pos: &PlayerVector3,
        factory: &Factory,
    ) {
        const GRID_SIZE_MAX: i16 = 20;

        let t = self.start_time.elapsed().as_secs_f32();
        let grid_size = if t >= 1.0 {
            GRID_SIZE_MAX
        } else {
            #[allow(clippy::cast_possible_truncation, reason = "this is intentional")]
            {
                ease::circ_in_out(
                    self.start_time.elapsed().as_secs_f32(),
                    0.0,
                    GRID_SIZE_MAX.into(),
                    1.0,
                ) as i16
            }
        };

        let origin = &factory.origin;
        let position_in_factory = player_pos.to_factory(origin).unwrap();

        let x_min = (position_in_factory.x - grid_size).max(factory.bounds.min.x);
        let x_max = (position_in_factory.x + grid_size).min(factory.bounds.max.x);
        let z_min = (position_in_factory.z - grid_size).max(factory.bounds.min.z);
        let z_max = (position_in_factory.z + grid_size).min(factory.bounds.max.z);

        for x in x_min..=x_max {
            d.draw_line3D(
                FactoryVector3 { x, y: 0, z: z_min }.to_player_relative(player_pos, origin),
                FactoryVector3 { x, y: 0, z: z_max }.to_player_relative(player_pos, origin),
                Color::RED,
            );
        }

        for z in z_min..=z_max {
            d.draw_line3D(
                FactoryVector3 { x: x_min, y: 0, z }.to_player_relative(player_pos, origin),
                FactoryVector3 { x: x_max, y: 0, z }.to_player_relative(player_pos, origin),
                Color::BLUE,
            );
        }
    }
}
