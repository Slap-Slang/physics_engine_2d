use std::fmt::Debug;

use cvector::cvector::{CVector, TooF64};

pub struct CParticle {
    position: CVector,
    velocity: CVector,
    acceleration: CVector,
}

impl Debug for CParticle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // f.debug_struct("CParticle")
        //     .field("pos: ({},{})", &self.position.x(), &self.position.y)
        //     .field("velocity", &self.velocity)
        //     .field("acceleration", &self.acceleration)
        //     .finish()
        println!("pos: ({},{})", &self.position.x(), &self.position.y());
        Ok(())
    }
}

impl CParticle {
    pub fn new<T>(pos_x: T, pos_y: T) -> Self
    where
        T: TooF64 + PartialOrd + Copy,
    {
        CParticle {
            position: CVector::new(pos_x, pos_y),
            velocity: CVector::new(0, 0),
            acceleration: CVector::new(0, 0),
        }
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
