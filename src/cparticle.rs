use std::fmt::Debug;

use cvector::cvector::{CVector, TooF64};

pub struct CParticle {
    pub position: CVector,
    pub velocity: CVector,
    pub acceleration: CVector,
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
        T: Copy + TooF64,
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

    pub fn set_vel<T>(&mut self, x: T, y: T)
    where
        T: Copy + TooF64,
    {
        self.velocity = CVector::new(x, y);
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
