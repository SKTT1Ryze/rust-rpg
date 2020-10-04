//! Cube Player Implement
//! 

use config::{
    typedef::{Pos, Color},
    size::{WINDOW_SIZE},
};
use super::{Player, Appearance};
use std::{fmt, fmt::Formatter};

pub struct CubePlayer {
    name: String,
    pos: Pos,
    color: Color,
    texture: (char, Color),
}

impl CubePlayer {
    pub fn new(name: &str, pos: Pos, color: Color, texture: (char, Color)) -> Self {
        Self {
            name: String::from(name),
            pos,
            color,
            texture,
        }
    }

    fn move_up(&mut self, step_len: u32) {
        println!("move up");
        if (self.pos[1] as u32) < step_len {
            self.pos[1] = 0f64;
        }
        else {
            self.pos[1] -= step_len as f64;
        }
    }

    fn move_down(&mut self, step_len: u32) {
        println!("move down");
        if (self.pos[1] as u32 + step_len) > WINDOW_SIZE {
            self.pos[1] = WINDOW_SIZE as f64;
        }
        else {
            self.pos[1] += step_len as f64;
        }
    }

    fn move_left(&mut self, step_len: u32) {
        println!("move left");
        if (self.pos[0] as u32 ) < step_len {
            self.pos[0] = 0f64;
        }
        else {
            self.pos[0] -= step_len as f64;
        }
    }

    fn move_right(&mut self, step_len: u32) {
        println!("move right");
        if (self.pos[0] as u32 + step_len) > WINDOW_SIZE {
            self.pos[0] = WINDOW_SIZE as f64;
        }
        else {
            self.pos[0] += step_len as f64;
        }
    }
    
}

impl Player for CubePlayer {
    fn name(&self) -> String {
        self.name.clone()
    }

    /// Transform: move one block  
    /// |direction[0]|direction[1]|move|
    /// |...|...|...|
    /// |0|0|Up|
    /// |0|1|Down|
    /// |1|0|Left|
    /// |1|1|Right|
    /// 
    fn transform(&mut self, direction: [u32; 2], step_len: u32) {
        match (direction[0] > 0, direction[1] > 0) {
            (false, false) => {
                // Move Up
                self.move_up(step_len);
            },
            (false, true) => {
                // Move Down
                self.move_down(step_len);
            },
            (true, false) => {
                // Move Left
                self.move_left(step_len);
            },
            (true, true) => {
                // Move Right
                self.move_right(step_len);
            },
        }
    }

    fn pos(&self) -> Pos {
        self.pos
    }

    fn appearance(&self) -> Appearance {
        Appearance::CubeAppearance(self.color[0], self.color[1], self.color[2], self.color[3], self.texture)
    }
    fn set_appearance(&mut self, apper: Appearance) {
        match apper {
            Appearance::CubeAppearance(x, y, j, k, c) => {
                self.color = [x, y, j ,k];
                self.texture = c;
            }
        }       
    }
}

impl fmt::Debug for CubePlayer {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let (color, texture) = match self.appearance() {
            Appearance::CubeAppearance(x, y, j, k, c) => ([x, y, j, k], c),
        };
        f.debug_struct("CubePlayer")
            .field("name", &self.name())
            .field("pos", &self.pos())
            .field("color", &color)
            .field("texture", &texture)
            .finish()
    }
}