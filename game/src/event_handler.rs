//! Event Handler for Rust-RPG
//! 

use piston::input::GenericEvent;
use error::Error;
use player::{CubePlayer, Player};
use config::{size::STEP_LEN, status::MouseStatus};
use super::object_manager::{Manager, cubes_manager::CubesManager};
use super::mouse_detection::MouseDetector;

pub trait EventHandler {
    fn handle_event<E: GenericEvent>(&mut self, e: &E)
        -> Result<(), Error>;
}

impl EventHandler for CubePlayer {
    fn handle_event<E: GenericEvent>(&mut self, e: &E) -> Result<(), Error> {
        use piston::input::{Button, ButtonState, Key};
        if let Some(k) = e.button_args() {
            if k.state == ButtonState::Press {
                match k.button {
                    Button::Keyboard(Key::Up) => self.transform([0,0], STEP_LEN),
                    Button::Keyboard(Key::Down) => self.transform([0,1], STEP_LEN),
                    Button::Keyboard(Key::Left) => self.transform([1,0], STEP_LEN),
                    Button::Keyboard(Key::Right) => self.transform([1,1], STEP_LEN),
                    _ => {},
                }
            }
        }
        Ok(())
    }
}

impl EventHandler for CubesManager {
    fn handle_event<E: GenericEvent>(&mut self, e: &E) -> Result<(), Error> {
        use piston::input::{Button, ButtonState, Key};
        if let Some(k) = e.button_args() {
            if k.state == ButtonState::Press {
                match k.button {
                    Button::Keyboard(Key::D1) => self.set_current_objecat(0),
                    Button::Keyboard(Key::D2) => self.set_current_objecat(1),
                    Button::Keyboard(Key::D3) => self.set_current_objecat(2),
                    Button::Keyboard(Key::D4) => self.set_current_objecat(3),
                    Button::Keyboard(Key::D5) => self.set_current_objecat(4),
                    Button::Keyboard(Key::D6) => self.set_current_objecat(5),
                    Button::Keyboard(Key::D7) => self.set_current_objecat(6),
                    Button::Keyboard(Key::D8) => self.set_current_objecat(7),
                    Button::Keyboard(Key::D9) => self.set_current_objecat(8),
                    Button::Keyboard(Key::Up) => {
                        match self.current_cube() {
                            Some(cube) => cube.transform([0, 0], STEP_LEN),
                            None => {},
                        }
                    },
                    Button::Keyboard(Key::Down) => {
                        match self.current_cube() {
                            Some(cube) => cube.transform([0, 1], STEP_LEN),
                            None => {},
                        }
                    },
                    Button::Keyboard(Key::Left) => {
                        match self.current_cube() {
                            Some(cube) => cube.transform([1, 0], STEP_LEN),
                            None => {},
                        }
                    },
                    Button::Keyboard(Key::Right) => {
                        match self.current_cube() {
                            Some(cube) => cube.transform([1, 1], STEP_LEN),
                            None => {},
                        }
                    },
                    _ => {},
                }
            }
        }
        Ok(())
    }
}

impl EventHandler for MouseDetector {
    fn handle_event<E: GenericEvent>(&mut self, e: &E) -> Result<(), Error> {
        use piston::{ButtonState, Button, MouseButton};
        if let Some(pos) = e.mouse_cursor_args() {
            self.pos = pos;
        }
        if let Some(m) = e.button_args() {
            if m.state == ButtonState::Press {
                match m.button {
                    Button::Mouse(MouseButton::Left) => {
                        self.status = MouseStatus::Left;
                        println!("Mouse Pos: [{}, {}]", self.pos[0], self.pos[1]);
                    },
                    Button::Mouse(MouseButton::Right) => {
                        self.status = MouseStatus::Right;
                        println!("Mouse Pos: [{}, {}]", self.pos[0], self.pos[1]);
                    },
                    Button::Mouse(MouseButton::Middle) => {
                        self.status = MouseStatus::Middle;
                        println!("Mouse Pos: [{}, {}]", self.pos[0], self.pos[1]);
                    },
                    _ => {},
                }
            }
        }
        Ok(())
    }
}