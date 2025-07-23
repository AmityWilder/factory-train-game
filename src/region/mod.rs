use crate::{player::Player, resource::Resources};
use factory::Factory;
use lab::Laboratory;
use raylib::prelude::*;

pub mod factory;
pub mod lab;

#[derive(Debug, Default)]
pub enum Region<'a> {
    #[default]
    Rail,
    Factory(&'a mut Factory),
    Lab(&'a mut Laboratory),
}

impl Region<'_> {
    pub fn draw(
        &self,
        d: &mut impl RaylibDraw3D,
        thread: &RaylibThread,
        resources: &Resources,
        player: &Player,
    ) {
        match self {
            Self::Rail => todo!(),
            Self::Factory(factory) => factory.draw(d, thread, resources, player),
            Self::Lab(lab) => lab.draw(d, thread, resources, player),
        }
    }
}
