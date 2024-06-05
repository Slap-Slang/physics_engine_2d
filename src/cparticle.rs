use cvector::cvector::{CVector, TooF64};

pub struct CParticle {
    position: CVector,
    velocity: CVector,
    acceleration: CVector,
}

impl CParticle {
    fn new<T>(pos_x: T, pos_y: T) -> Self
    where
        T: TooF64 + PartialOrd + Copy,
    {
        CParticle {
            position: CVector::new(pos_x, pos_x),
            velocity: CVector::new(0, 0),
            acceleration: CVector::new(0, 0),
        }
    }
}
