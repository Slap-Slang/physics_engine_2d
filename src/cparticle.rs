use std::fmt::Debug;

use cvector::cvector::{CVector, TooF64};

pub struct CParticle {
    position: CVector,
    velocity: CVector,
    acceleration: CVector,
}

impl Debug for CParticle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CParticle")
            .field(
                "position",
                &format!("({},{})", self.position.x(), self.position.y()),
            )
            .finish()
    }
}

impl CParticle {
    pub fn new<T>(pos_x: T, pos_y: T) -> Self
    where
        T: TooF64 + Copy,
    {
        CParticle {
            position: CVector::new(pos_x, pos_y),
            velocity: CVector::new(0, 0),
            acceleration: CVector::new(0, 0),
        }
    }

    pub fn x(&self) -> f64 {
        self.position.x()
    }

    pub fn y(&self) -> f64 {
        self.position.y()
    }

    pub fn pos(&self) -> (f64, f64) {
        (self.x(), self.y())
    }

    pub fn add_force(&mut self, f: &CVector) {
        self.acceleration.add(f);
    }

    pub fn apply_forces(&mut self) {
        self.velocity.add(&self.acceleration);
        self.acceleration.set_mag(0);
        self.position.add(&self.velocity);
    }
}
