//! Player Implement for Rust RPG
//! 

extern crate config;
use config::typedef::{Pos, Color};

mod cube;

pub use cube::CubePlayer;

pub trait Player {
    fn name(&self) -> String;
    fn transform(&mut self, direction: [u32; 2], step_len: u32);
    fn pos(&self) -> Pos;
    fn appearance(&self) -> Appearance;
    fn set_appearance(&mut self, apper: Appearance);
}

pub enum Appearance {
    CubeAppearance(f32, f32, f32, f32, (char, Color)),
}