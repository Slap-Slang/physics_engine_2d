use std::ops::{Deref, DerefMut};

use cvector::cvector::{CVector, TooF64};
use ratatui::{style::Color, widgets::canvas::Circle};

use crate::cparticle::CParticle;

pub struct CCircle {
    particle: CParticle,
    body: Circle,
}

impl CCircle {
    pub fn new<T>(x: T, y: T, radius: T) -> CCircle
    where
        T: TooF64 + Copy,
    {
        CCircle {
            particle: CParticle::new(x, y),
            body: Circle {
                x: x.too_f64(),
                y: y.too_f64(),
                radius: radius.too_f64(),
                color: Color::Yellow,
            },
        }
    }

    pub fn apply_forces(&mut self) {
        self.particle.apply_forces();
        (self.body.x, self.body.y) = self.particle.pos();
    }
}

impl Deref for CCircle {
    type Target = CParticle;

    fn deref(&self) -> &Self::Target {
        &self.particle
    }
}

impl DerefMut for CCircle {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.particle
    }
}
