use crate::vec3::Vec3;
use crate::vec3::Nm;

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}


impl Ray {

    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Ray { origin, direction }
    }

    fn point_at(&self, t: Nm) -> Vec3 {
        self.origin + (self.direction * t)
    }


}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn point_at() {
        let r = Ray {
            origin: Vec3::new(1., 2., 3.),
            direction: Vec3::new(0.1, 0.2, 0.3),
        };

        assert_eq!(
            r.point_at(0.),
            Vec3 { x: 1.0, y: 2.0, z: 3.0 });
        assert_eq!(
            r.point_at(4.),
            Vec3 { x: 1.4, y: 2.8, z: 4.2 }
        );
    }
}