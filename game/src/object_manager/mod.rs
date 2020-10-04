//! Object Manager Implement
//! 

pub mod cubes_manager;

/// Manager Trait
pub trait Manager {
    fn init(&mut self);
    fn set_current_objecat(&mut self, id: usize);
}
