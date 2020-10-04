//! Cubes Manager Implement
//! 

use player::{CubePlayer, Appearance, Player};
use super::Manager;

pub struct CubesManager {
    pub cubes: Vec<CubePlayer>,
    current_cube_index: Option<usize>,
    size: usize,
}

impl CubesManager {
    pub fn new(cubes: Vec<CubePlayer>) -> Self {
        let size = cubes.len();
        Self {
            cubes,
            current_cube_index: None,
            size,
        }
    }

    #[allow(unconditional_recursion)]
    pub fn current_cube(&mut self) -> Option<&mut CubePlayer> {
        match self.current_cube_index {
            Some(index) => Some(&mut self.cubes[index]),
            None => None,
        }
    }

}

impl Manager for CubesManager {
    fn init(&mut self) {
        self.current_cube_index = None;
        for cube in &mut self.cubes {
            cube.set_appearance(Appearance::CubeAppearance(
                config::color::RED[0],
                config::color::RED[1],
                config::color::RED[2],
                config::color::RED[3],
                ('*', config::color::BLACK),
            ));
        }
        self.size = self.cubes.len();
    }

    #[allow(unused_parens)]
    fn set_current_objecat(&mut self, index: usize) {
        if (index < self.cubes.len()) {
            self.current_cube_index = Some(index);
            println!("set current object with index [{}]", index);
        }
    }

    
}

