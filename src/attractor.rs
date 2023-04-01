use bevy::prelude::*;

#[derive(Default, Bundle, Clone)]
pub struct LorenzBundle {
    pub attractor: LorenzAttractor,
    pub mesh: Handle<Mesh>,
    pub material: Handle<StandardMaterial>,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub computed_visibility: ComputedVisibility,
}

pub trait Attractor {
    fn update(&self, p: &mut Vec3, dt: f32);
}

pub fn update_node_positions<A: Attractor + Component>(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &A)>,
) {
    let dt = time.delta_seconds();
    for (mut p, l) in &mut query {
        l.update(&mut p.translation, dt)
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
            rho: 30.0,
            beta: 8.0 / 3.0,
        }
    }
}

impl Attractor for LorenzAttractor {
    fn update(&self, p: &mut Vec3, dt: f32) {
        let d = Vec3 {
            x: (self.sigma * (p.y - p.x)),
            y: (p.x * (self.rho - p.z) - p.y),
            z: (p.x * p.y - self.beta * p.z),
        };
        *p += d * dt;
    }
}
