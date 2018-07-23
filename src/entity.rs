use cgmath::Vector3;

pub struct Entity {
    // (x, y, z)
    pub position: Vector3<f32>,
    pub rotation: Vector3<f32>,
}

impl Entity {
    pub fn from_position(pos: (f32, f32, f32)) -> Entity {
        Entity {
            position: Vector3::new(pos.0, pos.1, pos.2),
            rotation: Vector3::new(0.0, 0.0, 0.0),
        }
    }

    pub fn new(pos: Vector3<f32>) -> Entity {
        Entity {
            position: pos,
            rotation: Vector3::new(0.0, 0.0, 0.0),
        }
    }

    pub fn update_position(&mut self, delta_pos: Vector3<f32>) -> () {
        self.position += delta_pos;
    }
}
