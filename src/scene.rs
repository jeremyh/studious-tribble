use crate::hitable::{Hit, Hitable};
use crate::ray::Ray;
use crate::vec3::F;
use std::ops::Range;

pub struct Scene<'a> {
    things: Vec<Box<dyn Hitable + 'a>>,
}

impl<'a> Scene<'a> {
    pub fn new() -> Self {
        Scene { things: vec![] }
    }

    pub fn add(
        self: &mut Self,
        thing: Box<dyn Hitable + 'a>,
    ) {
        self.things.push(thing);
    }
}
impl Hitable for Scene<'_> {
    fn hit(
        self: &Self,
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
