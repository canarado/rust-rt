use std::f64;
use std::cmp::Ordering;
use crate::ray::Ray;
use crate::hittable::{Hit, HitRecord};
use crate::aabb::AABB;

enum BVHNode {
    Branch { left: Box<BVH>, right: Box<BVH> },
    Leaf(Box<dyn Hit>)
}

pub struct BVH {
    tree: BVHNode,
    bbox: AABB
}

impl BVH {
    pub fn new(mut hitable: Vec<Box<dyn Hit>>, time0: f64, time1: f64) -> Self {
        fn box_compare(time0: f64, time1: f64, axis: u8) -> impl FnMut(&Box<dyn Hit>, &Box<dyn Hit>) -> Ordering {
            move |a, b| {
                let a_bbox = a.bounding_box(time0, time1);
                let b_bbox = b.bounding_box(time0, time1);
                if let (Some(a), Some(b)) = (a_bbox, b_bbox) {
                    let ac = a.minimum[axis] + a.maximum[axis];
                    let bc = b.minimum[axis] + b.maximum[axis];
                    ac.partial_cmp(&bc).unwrap()
                } else {
                    panic!["no bounding box in bvh node"]
                }
            }
        }

        fn axis_range(hitable: &Vec<Box<dyn Hit>>, time0: f64, time1: f64, axis: u8) -> f64 {
            let (min, max) = hitable.iter().fold((f64::MAX, f64::MIN), |(bmin, bmax), hit| {
                if let Some(aabb) = hit.bounding_box(time0, time1) {
                    (bmin.min(aabb.minimum[axis]), bmax.max(aabb.maximum[axis]))
                } else {
                    (bmin, bmax)
                }
            });
            max - min
        }

        let mut axis_ranges: Vec<(u8, f64)> = (0..3)
            .map(|a| (a, axis_range(&hitable, time0, time1, a)))
            .collect();

        axis_ranges.sort_unstable_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        let axis = axis_ranges[0].0;

        hitable.sort_unstable_by(box_compare(time0, time1, axis));
        let len = hitable.len();
        match len {
            0 => panic!["no elements in scene"],
            1 => {
                let leaf = hitable.pop().unwrap();
                if let Some(bbox) = leaf.bounding_box(time0, time1) {
                    BVH { tree: BVHNode::Leaf(leaf), bbox }
                } else {
                    panic!["no bounding box in bvh node"]
                }
            },
            _ => {
                let right = BVH::new(hitable.drain(len / 2..).collect(), time0, time1);
                let left = BVH::new(hitable, time0, time1);
                let bbox = AABB::surrounding_box(&left.bbox, &right.bbox);
                BVH { tree: BVHNode::Branch { left: Box::new(left), right: Box::new(right) }, bbox }
            }
        }
    }
}

impl Hit for BVH {
    fn hit(&self, ray: &Ray, t_min: f64, mut t_max: f64) -> Option<HitRecord> {
        if self.bbox.hit(&ray, t_min, t_max) {
            match &self.tree {
                BVHNode::Leaf(leaf) => leaf.hit(&ray, t_min, t_max),
                BVHNode::Branch { left, right} => {
                    let left = left.hit(&ray, t_min, t_max);
                    if let Some(l) = &left { t_max = l.t };
                    let right = right.hit(&ray, t_min, t_max);
                    if right.is_some() { right } else { left }
                }
            }
        } else {
            None
        }
    }

    fn bounding_box(&self, _t0: f64, _t1: f64) -> Option<AABB> {
        Some(self.bbox)
    }
}