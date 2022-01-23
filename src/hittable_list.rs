use crate::hittable::*;
use crate::ray::*;

pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn push<T: Hittable + 'static>(&mut self, object: T) -> &mut Self {
	    self.objects.push(Box::new(object));
	    self
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut temp_rec: HitRecord = HitRecord::new();
        let mut hit_anything: bool = false;
        let mut closest_so_far: f64 = t_max;

        for object_box in self.objects.iter() {
	    let object = &**object_box;
            if object.hit(r, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
		*rec = temp_rec;
            }
        }

        hit_anything
    }
}
