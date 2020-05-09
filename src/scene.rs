use crate::hitable::{Hit, Hitable};
use crate::ray::Ray;
use std::ops::Range;

pub struct Scene {
    things: Vec<Box<dyn Hitable>>,
}

impl Scene {
    pub fn new() -> Self {
        Scene { things: vec![] }
    }

    pub fn add(
        self: &mut Self,
        thing: Box<dyn Hitable>,
    ) {
        self.things.push(thing);
    }
}
impl Hitable for Scene {
    fn hit(
        self: &Self,
        ray: &Ray,
        t: &Range<f32>,
    ) -> Option<Hit> {
        let mut hit = None;
        let mut closest_so_far: f32 = t.end;

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
