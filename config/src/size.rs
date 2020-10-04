//! Size Config
//! 

#![allow(dead_code)]
pub const WINDOW_SIZE: u32 = 1024;
pub const MAP_BLOCK_SIZE: u32 = 20;
pub const STEP_LEN: u32 = 10;
pub const CUBE_NUM: u32 = 3;

pub enum ObjectSize {
    CubeSize(u32, u32),
    SphereSize(u32),
}