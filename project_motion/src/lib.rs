const GRAVITY: f64 = 9.8;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct ThrownObject {
    initial_position: Vec2,
    initial_velocity: Vec2,
    elapsed: f64,
}

impl ThrownObject {
    pub fn new(position: Vec2, velocity: Vec2) -> Self {
        ThrownObject {
            initial_position: position,
            initial_velocity: velocity,
            elapsed: 0.0,
        }
    }
}

impl Iterator for ThrownObject {
    type Item = (Vec2, Vec2);

    fn next(&mut self) -> Option<Self::Item> {
        self.elapsed += 1.0;
        let t = self.elapsed;
        let (p, v) = (self.initial_position, self.initial_velocity);

        let position = Vec2 {
            x: p.x + v.x * t,
            y: p.y + v.y * t - GRAVITY * t * t / 2.0,
        };
        let velocity = Vec2 {
            x: v.x,
            y: v.y - GRAVITY * t,
        };

        if position.y <= 0.0 {
            return None;
        }
        Some((position, velocity))
    }
}
