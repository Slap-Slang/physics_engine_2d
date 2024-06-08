use std::ops::{Deref, DerefMut};

use cvector::cvector::{CVector, TooF64};
use ratatui::{layout::Rect, style::Color, widgets::canvas::Circle};

use crate::cparticle::CParticle;

pub struct CCircle {
    particle: CParticle,
    pub body: Circle,
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

    pub fn bounce(&mut self, bounds: &Rect) {
        if self.particle.x() + self.body.radius >= bounds.right() as f64
            || self.particle.x() - self.body.radius <= bounds.left() as f64
        {
            self.velocity.mult_x(-1);
        }

        if self.particle.y() + self.body.radius >= bounds.bottom() as f64
            || self.particle.y() - self.body.radius <= bounds.top() as f64
        {
            self.velocity.mult_y(-1);
        }
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
