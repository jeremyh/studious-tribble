use crate::hitable::{Hit, Hitable};
use crate::ray::Ray;
use crate::vec3::F;
use std::ops::Range;

pub struct Scene {
    things: Vec<Box<dyn Hitable + Send + Sync>>,
}

impl Scene {
    pub fn new() -> Self {
        Scene { things: vec![] }
    }

    pub fn add(
        &mut self,
        thing: Box<dyn Hitable + Send + Sync>,
    ) {
        self.things.push(thing);
    }
}
impl Hitable for Scene {
    fn hit(
        &self,
        ray: &Ray,
        t: &Range<F>,
    ) -> Option<Hit> {
        let mut hit = None;
        let mut closest_so_far: F = t.end;

        for thing in &self.things {
            if let Some(h) = thing
                .hit(ray, &(t.start..closest_so_far))
            {
                closest_so_far = h.t;
                hit = Some(h);
            }
        }
        hit
    }
}
