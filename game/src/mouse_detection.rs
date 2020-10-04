//! Mouse Input Detection
//! 

use config::{
    typedef::Pos,
    status::MouseStatus,
};

pub struct MouseDetector {
    pub pos: Pos,
    pub status: MouseStatus,
}

impl MouseDetector {
    pub fn new() -> Self {
        Self {
            pos: [0f64, 0f64],
            status: MouseStatus::Release,
        }
    }
}
