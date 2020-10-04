//! Block Implement
//! 

use config::typedef::*;

pub struct Block {
    pub pos: Pos,
    pub size: BlockSize,
    pub color: Color,
}

impl Block {
    pub fn new(pos: Pos, size: BlockSize, color: Color) -> Self {
        Self {
            pos,
            size,
            color,
        }
    }

}