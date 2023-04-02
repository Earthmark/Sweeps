use bevy::prelude::*;

pub trait Attractor {
    fn delta(&self, p: Vec3) -> Vec3;
}

pub fn update_node_positions<A: Attractor + Component>(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &A)>,
) {
    let dt = time.delta_seconds();
    for (mut p, l) in &mut query {
        let pos = p.translation;
        p.translation += l.delta(pos) * dt;
    }
}

#[derive(Component, Clone)]
pub struct LorenzAttractor {
    sigma: f32,
    rho: f32,
    beta: f32,
}

impl Default for LorenzAttractor {
    fn default() -> Self {
        Self {
            sigma: 3.0,
            rho: 28.0,
            beta: 8.0 / 3.0,
        }
    }
}

impl Attractor for LorenzAttractor {
    fn delta(&self, p: Vec3) -> Vec3 {
        Vec3 {
            x: (self.sigma * (p.y - p.x)),
            y: (p.x * (self.rho - p.z) - p.y),
            z: (p.x * p.y - self.beta * p.z),
        }
    }
}

#[derive(Component, Clone)]
pub struct ThreeCellCnnAttractor {
    p0: f32,
    p1: f32,
    p2: f32,
    p3: f32,
}

impl Default for ThreeCellCnnAttractor {
    fn default() -> Self {
        Self {
            p0: 1.24,
            p1: 1.1,
            p2: 4.4,
            p3: 3.21,
        }
    }
}

impl Attractor for ThreeCellCnnAttractor {
    fn delta(&self, p: Vec3) -> Vec3 {
        fn shift(h: f32) -> f32 {
            0.5 * ((h + 1.).abs() - (h - 1.).abs())
        }
        let hx = shift(p.x);
        let hy = shift(p.y);
        let hz = shift(p.z);
        Vec3 {
            x: -p.x + self.p0 * hx - self.p3 * hy - self.p3 * hz,
            y: -p.y - self.p3 * hx + self.p1 * hy - self.p2 * hz,
            z: -p.z - self.p3 * hx + self.p2 * hy + hz,
        }
    }
}

#[derive(Component, Clone)]
pub struct AizawaAttractor {
    p0: f32,
    p1: f32,
    p2: f32,
    p3: f32,
    p4: f32,
    p5: f32,
}

impl Default for AizawaAttractor {
    fn default() -> Self {
        Self {
            p0: 0.95,
            p1: 0.7,
            p2: 0.6,
            p3: 3.5,
            p4: 0.25,
            p5: 0.1,
        }
    }
}

impl Attractor for AizawaAttractor {
    fn delta(&self, p: Vec3) -> Vec3 {
        Vec3 {
            x: (p.z - self.p1) * p.x - self.p3 * p.y,
            y: self.p3 * p.x + (p.z - self.p1) * p.y,
            z: self.p2 + self.p0 * p.z
                - (p.z * p.z * p.z) / 3.
                - (p.x * p.x + p.y * p.y) * (1. + self.p4 * p.z)
                + self.p5 * p.z * p.x * p.x * p.x,
        }
    }
}

#[derive(Component, Clone)]
pub struct BoualiAttractor {
    p0: f32,
    p1: f32,
}

impl Default for BoualiAttractor {
    fn default() -> Self {
        Self { p0: 0.3, p1: 1.0 }
    }
}

impl Attractor for BoualiAttractor {
    fn delta(&self, p: Vec3) -> Vec3 {
        Vec3 {
            x: p.x * (4. - p.y) + self.p0 * p.z,
            y: -p.y * (1. - p.x * p.x),
            z: -p.x * (1.5 - p.z * self.p1) - 0.05 * p.z,
        }
    }
}

#[derive(Component, Clone)]
pub struct ChenLeeAttractor {
    p0: f32,
    p1: f32,
    p2: f32,
}

impl Default for ChenLeeAttractor {
    fn default() -> Self {
        Self {
            p0: 5.,
            p1: -10.,
            p2: -0.38,
        }
    }
}

impl Attractor for ChenLeeAttractor {
    fn delta(&self, p: Vec3) -> Vec3 {
        Vec3 {
            x: self.p0 * p.x - p.y * p.z,
            y: self.p1 * p.y + p.x * p.z,
            z: self.p2 * p.z + p.x * p.y / 3.,
        } * 0.25
    }
}

#[derive(Component, Clone)]
pub struct HalvorsenAttractor {
    p0: f32,
}

impl Default for HalvorsenAttractor {
    fn default() -> Self {
        Self { p0: 1.4 }
    }
}

impl Attractor for HalvorsenAttractor {
    fn delta(&self, p: Vec3) -> Vec3 {
        let t = |s: f32, m: f32, l: f32| -> f32 { self.p0 * s - 4. * m - 4. * l - m * m };
        Vec3 {
            x: t(p.x, p.y, p.z),
            y: t(p.y, p.z, p.x),
            z: t(p.z, p.x, p.y),
        }
    }
}

#[derive(Component, Clone)]
pub struct FinanceAttractor {
    p0: f32,
    p1: f32,
    p2: f32,
}

impl Default for FinanceAttractor {
    fn default() -> Self {
        Self {
            p0: 0.001,
            p1: 0.2,
            p2: 1.1,
        }
    }
}

impl Attractor for FinanceAttractor {
    fn delta(&self, p: Vec3) -> Vec3 {
        Vec3 {
            x: (1. / self.p1 - self.p0) * p.x + p.z + p.x * p.y,
            y: -self.p1 * p.y - p.x * p.x,
            z: -p.x - self.p2 * p.z,
        }
    }
}

#[derive(Component, Clone)]
pub struct NewtonLeipnikAttractor {
    p0: f32,
    p1: f32,
}

impl Default for NewtonLeipnikAttractor {
    fn default() -> Self {
        Self { p0: 0.4, p1: 0.175 }
    }
}

impl Attractor for NewtonLeipnikAttractor {
    fn delta(&self, p: Vec3) -> Vec3 {
        Vec3 {
            x: -self.p0 * p.x + p.y + 10. * p.y * p.z,
            y: -p.x - 0.4 * p.y + 5. * p.x * p.z,
            z: self.p1 * p.z - 5. * p.x * p.y,
        }
    }
}

#[derive(Component, Clone)]
pub struct NoseHooverAttractor {
    p0: f32,
}

impl Default for NoseHooverAttractor {
    fn default() -> Self {
        Self { p0: 1.5 }
    }
}

impl Attractor for NoseHooverAttractor {
    fn delta(&self, p: Vec3) -> Vec3 {
        Vec3 {
            x: p.y,
            y: -p.x + p.y * p.z,
            z: self.p0 - p.y * p.y,
        }
    }
}

#[derive(Component, Clone)]
pub struct ThomasAttractor {
    p0: f32,
}

impl Default for ThomasAttractor {
    fn default() -> Self {
        Self { p0: 0.2 }
    }
}

impl Attractor for ThomasAttractor {
    fn delta(&self, p: Vec3) -> Vec3 {
        Vec3 {
            x: -self.p0 * p.x + p.y.sin(),
            y: -self.p0 * p.y + p.z.sin(),
            z: -self.p0 * p.z + p.x.sin(),
        }
    }
}
