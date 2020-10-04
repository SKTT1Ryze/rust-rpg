//! Map Manager for Rust-RPG
//! 

extern crate config;

mod block;


use block::Block;
use config::{
    color::*,
    typedef::BlockSize,
};

pub struct Map {
    map: Vec<Vec<Block>>,
    pub block_size: BlockSize,
    pub map_size: u32,
}

impl Map {
    pub fn new(window_size: u32, block_size: u32) -> Self {
        let mut map = Vec::new();
        let map_size = window_size / block_size as u32;
        let mut current_color;
        let mut current_color_type = 0;
        for i in 0..map_size {
            let mut new_block_line = Vec::new();
            for j in 0..map_size {
                match current_color_type {
                    0 => {
                        current_color_type = 1;
                        current_color = YELLOW;
                    },
                    1 => {
                        current_color_type = 0;
                        current_color = WHITE;
                    },
                    _ => panic!("Map Faild"),
                }

                new_block_line.push(Block::new([(block_size * i) as f64, (block_size * j) as f64], block_size, current_color));
            }
            map.push(new_block_line);
        }
        
        Self {
            map,
            block_size,
            map_size,
        }
    }

    pub fn get_block(&self, index_x: usize, index_y: usize) -> &Block {
        &self.map[index_x][index_y]
    }
    
}


#[cfg(test)]
mod tests {
    #[test]
    fn make_map() {
        
    }
}
